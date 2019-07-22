use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Intern(usize);

#[derive(Debug)]
pub struct Interns<'proto>(Option<&'proto Interns<'proto>>, BaseInterns);

#[derive(Debug)]
struct BaseInterns {
    pub base_address: usize,
    pub to_string: Vec<String>,
    pub to_intern: HashMap<String, usize>,
}

impl BaseInterns {
    pub fn new(base_address: usize) -> BaseInterns {
        BaseInterns {
            base_address,
            to_string: vec![],
            to_intern: HashMap::new(),
        }
    }
}

impl<'proto> Interns<'proto> {
    pub fn new(base_address: usize) -> Interns<'static> {
        Interns(None, BaseInterns::new(base_address))
    }

    pub fn extend(&self) -> Interns {
        Interns(Some(self), BaseInterns::new(self.1.base_address + self.1.to_string.len()))
    }

    pub fn intern(&mut self, s: &str) -> Intern {
        if let Some(i) = self.to_intern(s) { return i; }

        let bi = &mut self.1;
        let id = bi.to_string.len();
        bi.to_string.push(s.to_string());
        bi.to_intern.insert(s.to_string(), id);
        return Intern(id + bi.base_address);
    }

    pub fn to_intern(&self, s: &str) -> Option<Intern> {
        match self {
            Interns(sprior, bi) => { 
                if let Some(prior) = sprior {
                    if let Some(i) = prior.to_intern(s) { return Some(i); }
                }
                bi.to_intern.get(s).map(|i| Intern(*i + bi.base_address)) 
            }
        }
    }

    pub fn to_string(&self, i: Intern) -> Option<&str> {
        match self {
            Interns(sprior, bi) => { 
                if let Some(prior) = sprior {
                    if let Some(s) = prior.to_string(i) { return Some(s); }
                }
                if i.0 < bi.base_address { return None; }
                bi.to_string.get(i.0 - bi.base_address).map(|i| i.as_str())
            }
        }
    }
}

impl Intern {
    pub fn raw(&self) -> usize { self.0 }
}