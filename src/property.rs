#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(&'static str),
    DoubleArray(&'static [f64]),
    None,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: &'static str,
    pub value: PropertyValue,
    pub tied: bool,
    pub children: Vec<Option<Box<Property>>>,
}

impl Property {
    pub fn leaf(name: &'static str, value: PropertyValue) -> Self {
        Self {
            name,
            value,
            tied: false,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Property) {
        self.children.push(Some(Box::new(child)));
    }

    pub fn child(&self, name: &str) -> Option<&Property> {
        self.children.iter().flatten().find(|c| c.name == name).map(|v| v.as_ref())
    }

    pub fn child_mut(&mut self, name: &str) -> Option<&mut Property> {
        self.children.iter_mut().flatten().find(|c| c.name == name).map(|b| b.as_mut())
    }

    pub fn value(&self) -> PropertyValue {
        self.value
    }

    pub fn set_value(&mut self, value: PropertyValue, _fire: bool) {
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn child_lookup_works() {
        let mut root = Property::leaf("root", PropertyValue::Float(0.0));
        root.add_child(Property::leaf("child", PropertyValue::Float(1.0)));
        assert_eq!(root.child("child").map(|c| c.value()), Some(PropertyValue::Float(1.0)));
    }

    #[test]
    fn set_value_updates_leaf() {
        let mut p = Property::leaf("p", PropertyValue::Float(0.0));
        p.set_value(PropertyValue::Float(5.0), false);
        assert_eq!(p.value(), PropertyValue::Float(5.0));
    }
}
