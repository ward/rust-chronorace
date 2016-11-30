/// Athlete should have the properties that eventually are placed into the CSV file that will be
/// generated. For this, we probably just want to use all the properties that stratenlopen.be
/// expects.
#[derive(Debug)]
pub struct Athlete {
    pub rank: u32, // required
    pub bib: Option<u32>,
    pub name: String, // Last name first name, required
    pub gender: Gender, // Male / female, required
    pub guntime: u32, // Seconds, required
    pub year_of_birth: Option<u16>,
    pub date_of_birth: Option<String>, // yyyy-mm-dd
    pub location: Option<String>,
    pub chiptime: Option<u32>, // Seconds
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female,
}

/// Uses Builder pattern for creation of an instance.
/// https://aturon.github.io/ownership/builders.html
impl Athlete {
    pub fn new() -> Athlete {
        Athlete {
            rank: 0,
            bib: None,
            name: "".to_string(),
            gender: Gender::Male,
            guntime: 0,
            year_of_birth: None,
            date_of_birth: None,
            location: None,
            chiptime: None,
        }
    }

    pub fn rank(&mut self, rank: u32) -> &Athlete {
        self.rank = rank;
        self
    }

    pub fn bib(&mut self, bib: u32) -> &Athlete {
        self.bib = Some(bib);
        self
    }

    pub fn name(&mut self, name: String) -> &Athlete {
        self.name = name;
        self
    }

    pub fn gender(&mut self, gender: Gender) -> &Athlete {
        self.gender = gender;
        self
    }

    pub fn guntime(&mut self, time: u32) -> &Athlete {
        self.guntime = time;
        self
    }

    pub fn year_of_birth(&mut self, year: u16) -> &Athlete {
        self.year_of_birth = Some(year);
        self
    }

    pub fn date_of_birth(&mut self, date: String) -> &Athlete {
        self.date_of_birth = Some(date);
        self
    }

    pub fn location(&mut self, location: String) -> &Athlete {
        self.location = Some(location);
        self
    }

    pub fn chiptime(&mut self, time: u32) -> &Athlete {
        self.chiptime = Some(time);
        self
    }

    // TODO? A from_str static method
}

/*
#[test]
fn build_athlete() {
    let athlete = Athlete::new()
                            .rank(10)
                            .bib(1234)
                            .name("Muylaert Ward".to_string())
                            .gender(Gender::Female)
                            .guntime(1000)
                            .year_of_birth(1989)
                            .date_of_birth("1989-04-24".to_string())
                            .location("Brussels, Belgium".to_string())
                            .chiptime(950);

    assert_eq!(athlete.rank, 10);
    assert_eq!(athlete.bib, Some(1234));
    assert_eq!(athlete.name, "Muylaert Ward");
    assert_eq!(athlete.gender, Gender::Female);

    // Testing how the entire thing looks with print and break
    //println!("{:?}", athlete);
    //assert!(false);
}
*/
