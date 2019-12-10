
pub struct Planet<'a> {
    name: &'a String,
    orbits: Option<&'a Planet<'a>>,
//    orbited_by: Vec<&'b Planet<'a>>
}

/*
impl<'a> Planet<'a> {
    fn new(self, orbits: Option<&'a Planet>) -> Planet<'a> {
        Planet {
            orbits,
            orbited_by: Vec::<&'a Planet>::new(),
        }
    }
}
*/

impl<'a> Planet<'a> {
    pub fn new(name: &'a String, orbits: Option<&'a Planet<'a>>) -> Planet<'a> {
        Planet {
            name,
            orbits
        }
    }
}
