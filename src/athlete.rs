/// Athlete should have the properties that eventually are placed into the CSV file that will be
/// generated. For this, we probably just want to use all the properties that stratenlopen.be
/// expects.
#[derive(Debug)]
pub struct Athlete {
    pub rank: u32, // required
    pub bib: Option<u32>,
    pub name: String,    // Last name first name, required
    pub gender: Gender,  // Male / female, required
    pub guntime: String, // Laziness just copies the string, required
    pub year_of_birth: Option<u16>,
    pub date_of_birth: Option<String>, // yyyy-mm-dd
    pub location: Option<String>,
    pub chiptime: Option<String>, // Laziness, just copy the string
}

#[derive(Debug, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

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
}

/// Trait to avoid some code duplication in building the final CSV file.
/// For the basic types this should just be the representation within the CSV file.
/// For the final row, this is a combination with the separator and such.
pub trait ToCSV {
    // Perhaps cleaner/clearer with a ToCSVElement and ToCSVRow?
    /// Turn this type into its CSV representation
    fn to_csv(&self) -> String;
}
impl ToCSV for Gender {
    fn to_csv(&self) -> String {
        match self {
            Gender::Male => "M".to_owned(),
            Gender::Female => "F".to_owned(),
        }
    }
}
impl ToCSV for u32 {
    fn to_csv(&self) -> String {
        self.to_string()
    }
}
impl ToCSV for u16 {
    fn to_csv(&self) -> String {
        self.to_string()
    }
}
impl ToCSV for String {
    fn to_csv(&self) -> String {
        self.to_owned()
    }
}
impl<T: ToCSV> ToCSV for Option<T> {
    fn to_csv(&self) -> String {
        match self {
            Some(x) => x.to_csv(),
            None => "".to_owned(),
        }
    }
}
impl ToCSV for Athlete {
    fn to_csv(&self) -> String {
        // Plaats;Borstnummer;Achternaam Voornaam;Geslacht (M/F);Bruto tijd (hh:mm:ss);geboortejaar
        // (yyyy);geboortedatum(yyyy-mm-dd);woonplaats;Netto tijd
        vec![
            self.rank.to_csv(),
            self.bib.to_csv(),
            self.name.to_csv(),
            self.gender.to_csv(),
            self.guntime.to_csv(),
            self.year_of_birth.to_csv(),
            self.date_of_birth.to_csv(),
            self.location.to_csv(),
            self.chiptime.to_csv(),
        ].join(";")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(
            athlete.to_csv(),
            "10;;Muylaert Ward;F;0:39:45;;;Brussels, Belgium;0:39:44"
        );
    }
}
