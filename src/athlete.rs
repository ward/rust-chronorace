/// Athlete should have the properties that eventually are placed into the CSV file that will be
/// generated. For this, we probably just want to use all the properties that stratenlopen.be
/// expects.
#[derive(Debug)]
pub struct Athlete {
    pub rank: u32, // required
    pub bib: Option<u32>,
    pub name: String, // Last name first name, required
    pub gender: Gender, // Male / female, required
    pub guntime: String, // Laziness just copies the string, required
    pub year_of_birth: Option<u16>,
    pub date_of_birth: Option<String>, // yyyy-mm-dd
    pub location: Option<String>,
    pub chiptime: Option<String>, // Laziness, just copy the string
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
            guntime: "".to_string(),
            year_of_birth: None,
            date_of_birth: None,
            location: None,
            chiptime: None,
        }
    }

    // Plaats;Borstnummer;Achternaam Voornaam;Geslacht (M/F);Bruto tijd (hh:mm:ss);geboortejaar
    // (yyyy);geboortedatum(yyyy-mm-dd);woonplaats;Netto tijd
    pub fn to_csv(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.rank.to_string());
        result.push(';');

        match self.bib {
            Some(bib) => result.push_str(&bib.to_string()),
            None => {}
        };

        result.push(';');
        result.push_str(&self.name);
        result.push(';');

        match self.gender {
            Gender::Male => result.push('M'),
            Gender::Female => result.push('F'),
        };

        result.push(';');
        result.push_str(&self.guntime);
        result.push(';');

        match self.year_of_birth {
            Some(year) => result.push_str(&year.to_string()),
            None => {}
        };

        result.push(';');

        match self.date_of_birth {
            Some(ref date) => result.push_str(&date),
            None => {}
        }

        result.push(';');

        match self.location {
            Some(ref loc) => result.push_str(&loc),
            None => {}
        }

        result.push(';');

        match self.chiptime {
            Some(ref time) => result.push_str(&time),
            None => {}
        }

        return result;
    }
}

#[test]
fn test_to_csv() {
    let mut athlete = Athlete::new();
    assert_eq!(athlete.to_csv(), "0;;;M;;;;;");
    athlete.rank = 10;
    athlete.name = "Muylaert Ward".to_string();
    athlete.gender = Gender::Female;
    athlete.guntime = "0:39:45".to_string();
    assert_eq!(athlete.to_csv(), "10;;Muylaert Ward;F;0:39:45;;;;");
    athlete.location = Some("Brussels, Belgium".to_string());
    athlete.chiptime = Some("0:39:44".to_string());
    assert_eq!(athlete.to_csv(),
               "10;;Muylaert Ward;F;0:39:45;;;Brussels, Belgium;0:39:44");
}
