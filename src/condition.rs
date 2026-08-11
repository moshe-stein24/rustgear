use crate::property::PropertyValue;

#[derive(Debug, Clone, Copy)]
pub struct Condition<'a> {
    pub expr: &'a str,
}

impl<'a> Condition<'a> {
    pub const fn new(expr: &'a str) -> Self {
        Self { expr }
    }

    pub fn evaluate(&self, root: &crate::property::Property) -> bool {
        let expr = self.expr.trim();
        let ops = ["==", "!=", "<=", ">=", "<", ">"];
        for op in ops {
            if let Some((lhs, rhs)) = split_once(expr, op) {
                let lhs = lhs.trim();
                let rhs = rhs.trim();
                if let Some(prop) = find_prop(root, lhs) {
                    return compare(prop.value(), rhs, op);
                }
            }
        }
        false
    }
}

fn split_once<'a>(s: &'a str, delim: &str) -> Option<(&'a str, &'a str)> {
    let idx = s.find(delim)?;
    Some((&s[..idx], &s[idx + delim.len()..]))
}

fn find_prop<'a>(root: &'a crate::property::Property, name: &str) -> Option<&'a crate::property::Property> {
    if root.name == name { return Some(root); }
    for child in root.children.iter().flatten() {
        if let Some(m) = find_prop(child, name) { return Some(m); }
    }
    None
}

fn compare(lhs: PropertyValue, rhs: &str, op: &str) -> bool {
    match (lhs, rhs.parse::<f64>()) {
        (PropertyValue::Float(a), Ok(b)) => match op {
            "==" => (a - b).abs() < f64::EPSILON,
            "!=" => (a - b).abs() >= f64::EPSILON,
            "<=" => a <= b,
            ">=" => a >= b,
            "<" => (a) < (b),
            ">" => (a) > (b),
            _ => false,
        },
        (PropertyValue::Int(a), Ok(b)) => match op {
            "==" => a as f64 == b,
            "!=" => a as f64 != b,
            "<=" => a as f64 <= b,
            ">=" => a as f64 >= b,
            "<" => (a as f64) < (b),
            ">" => (a as f64) > (b),
            _ => false,
        },
        (PropertyValue::Bool(a), Err(_)) if op == "==" => a == (rhs == "true"),
        (PropertyValue::Bool(a), Err(_)) if op == "!=" => a != (rhs == "true"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::Property;
    #[test]
    fn evaluate_true_equality() {
        let root = Property::leaf("gear-down", PropertyValue::Float(1.0));
        assert!(Condition::new("gear-down == 1.0").evaluate(&root));
    }

    #[test]
    fn evaluate_false_inequality() {
        let root = Property::leaf("gear-down", PropertyValue::Float(1.0));
        assert!(!Condition::new("gear-down == 0.0").evaluate(&root));
    }
}
