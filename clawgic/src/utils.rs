/// Returns whether the given string is a valid var name
pub fn is_valid_var_name(var: &str) -> bool{
    let name = var.trim().to_string();
    let mut chars = name.chars();
    let first = chars.next();
    if first.is_none_or(|c| !c.is_lowercase()){
        return false;
    }

    for c in chars{
        if !c.is_numeric(){
            return false;
        }
    }

    true
}

/// Returns whether the given string is a valid predicate name
pub fn is_valid_predicate_name(name: &str) -> bool{
    let name = name.trim().to_string();
    let mut chars = name.chars();
    let first = chars.next();
    if first.is_none_or(|c| !c.is_uppercase()){
        return false;
    }

    for c in chars{
        if !c.is_numeric(){
            return false;
        }
    }

    true
}