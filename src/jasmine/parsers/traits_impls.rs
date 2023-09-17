use super::*;

pub struct Impl;

impl ParseToTree for Impl {
    fn parse_to_tree(parser: &mut Parser, pair: Pair<'_, Rule>) -> Id {
        let line_col = pair.line_col();
        let inner = pair.into_inner();

        // output vars
        let mut for_trait = None;
        let for_type = inner
            .clone()
            .filter(|n| n.as_rule() == Rule::ident)
            .next()
            .map(|n| n.as_str().to_string());

        let mut functions = Vec::new();
        let item_id = cuid();

        // 1. before anything, traverse to scope
        let scope_id = match parser
            .implable(for_type.as_ref().unwrap())
            .unwrap_or_else(|| panic!("Could not find this struct or enum, at {:?}", line_col))
        {
            TreeItem::Structure { scope, .. } => *scope,
            TreeItem::Enumeration { scope, .. } => *scope,
            _ => unreachable!(),
        };

        parser.set_scope(scope_id);

        for rule in inner {
            match rule.as_rule() {
                Rule::explicit_ty => for_trait = Some(ExplicitType::parse_to_self(parser, rule)),
                Rule::impl_fn_def => functions.push(Function::parse_to_tree(parser, rule)),
                _ => {}
            }
        }

        let trait_ident = for_trait.map(|n| {
            if let ExplicitType::Custom(n) = n {
                n
            } else if let ExplicitType::WithGeneric { outer, .. } = n {
                let ExplicitType::Custom(n) = *outer else {
                    panic!("Incorrect trait for impl, at {:?}", line_col)
                };

                n
            } else {
                panic!("Incorrect trait for impl, at {:?}", line_col)
            }
        });

        let trait_id = trait_ident
            .as_ref()
            .map(|n| parser.ident(n))
            .flatten()
            .map(|n| n.id())
            .copied();

        let implable_mut = parser.implable_mut(for_type.as_ref().unwrap()).unwrap();

        let impl_tree = TreeItem::Implementation {
            item_id,
            trait_id: if trait_ident.is_some() {
                Some(trait_id.unwrap_or_else(|| panic!("Could not find trait, at {:?}", line_col)))
            } else {
                None
            },
            type_id: *implable_mut.id(),
            functions,
        };

        match implable_mut {
            TreeItem::Structure { impls, .. } => impls.push(item_id),
            TreeItem::Enumeration { impls, .. } => impls.push(item_id),
            _ => unreachable!(),
        }

        parser.add_tree_item(impl_tree, false);

        item_id
    }
}

pub struct Trait;
