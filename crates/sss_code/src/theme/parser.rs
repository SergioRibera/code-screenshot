use std::collections::HashMap;

use syntect::highlighting::Color;

use super::color::str_to_color;

pub type VimHighlight = (Option<Color>, Option<Color>, Option<String>);

pub fn parse_vim(vim: &str) -> HashMap<&str, VimHighlight> {
    vim.split(';')
        .map(|group| {
            tracing::debug!("Parsing Vim Group: {group:?}");
            let mut values = group.splitn(4, ',');
            let name = values.next().unwrap();
            let bg = values.next().and_then(str_to_color);
            let fg = values.next().and_then(str_to_color);
            let style = values
                .next()
                .and_then(|v| (!v.is_empty()).then_some(v.to_string()));
            (name, (fg, bg, style))
        })
        .collect::<HashMap<&str, VimHighlight>>()
}

const VIM_NAMES: [(&str, &str); 19] = [
    ("Number", "constant.numeric"),
    ("TSCharacter,Character", "constant.character"),
    ("String", "string"),
    ("Constant", "constant"),
    ("Identifier", "variable"),
    ("Keyword", "keyword, storage"),
    ("Comment", "comment"),
    ("Operator", "keyword.operator, operator"),
    ("Statement", "variable.parameter.function"),
    ("Type", "entity.name.class, meta.class, support.class, type, typeParameter, entity.type.name, entity.name.type, meta.type.name"),
    ("Structure", "enum, struct"),
    ("StorageClass", "support"),
    ("Function", "entity.name.function, support.function, function"),
    ("Macro", "macro, entity.name.function.macro"),
    ("TSField", "property"),
    ("TSParameter", "parameter"),
    ("Delimiter", "brace"),
    ("Conditional", "conditional, keyword.conditional, keyword.control.conditional"),
    ("MyTag", "brackethighlighter.tag, brackethighlighter.angle, brackethighlighter.round, brackethighlighter.square"),
];

pub fn vim_to_scope_str(v: &str) -> Option<&str> {
    VIM_NAMES
        .iter()
        .find(|(n, _)| n.contains(v))
        .inspect(|s| tracing::trace!("Scope {v:?} is: {s:?}"))
        .map(|(_, v)| *v)
}
