use athlete;

pub fn parse_athletes(content: &str) -> Vec<athlete::Athlete> {
    // 1. Check what headers are present in the page
    // 2. Link them to the properties we use
    // 3. Parse the athlete results to get the actual values, creating Athlete and adding to the
    //    Vec.
    let headerstartidx = content.find("<tr class=\"HeaderTitreClassement\">").unwrap();
    let (_, headeronwards) = content.split_at(headerstartidx);
    let endtable = headeronwards.find("</table>").unwrap();
    let (resultstable, _) = headeronwards.split_at(endtable);

    // Holds the indices of the columns we are interested in.
    // These are located in the header row and then used for every result row.
    let mut interestingcolumns = Vec::new();

    // Holds the athletes at the end
    let mut athletes = Vec::new();

    for row in resultstable.split("</tr>") {

        // In the header, we check which columns have the information we
        // are interested in. We need to do this since the columns are not
        // consistent across multiple results pages...
        if row.contains("<tr class=\"HeaderTitreClassement\">") {
            let mut namefound = false;
            for (i, cell) in row.split("</td>").enumerate() {
                // rfind instead of find, the first one will have both
                // <tr> tag and <td> tag
                if let Some(idx) = cell.rfind('>') {
                    let (_, text) = cell.split_at(idx + 1);
                    match text {
                        "" => {
                            // Assumes the first two are always Pos and Nr
                            // and followed by Gender (no header)
                            if interestingcolumns.len() == 2 {
                                interestingcolumns.push(i);
                            }
                        }
                        "Pos" | "Nr" | "Age" | "Time" | "City" | "Leef." | "Tijd" | "Gemeente" => {
                            interestingcolumns.push(i);
                        }
                        "Name" | "Naam" => {
                            // Name can also be in the category column.
                            // We only care about the first one.
                            if !namefound {
                                interestingcolumns.push(i);
                                namefound = true;
                            }
                        }
                        _ => {}
                    };
                }
            }
        }

        assert_eq!(interestingcolumns.len(), 7);

        // Once we reach the rows, we can focus on just those indices.
        // The assumption here is that their order will always be the same.
        // That is: Pos, Nr, Gender, Name, Age, Time, City
        if row.contains("<tr class=\"Even\"") || row.contains("<tr class=\"Odd\"") {
            // println!("Athlete unparsed: {}", row);
            let cells: Vec<&str> = row.split("</td>").collect();

            let mut athlete = athlete::Athlete::new();

            // Rank
            let mut rank = strip_tags(cells[interestingcolumns[0]].to_string());
            let ranklast = rank.len() - 1;
            rank.remove(ranklast);
            match rank.parse() {
                Ok(r) => athlete.rank = r,
                _ => continue,
            };
            // athlete.rank = rank.parse().unwrap();
            // TODO Not everyone has a rank... See how to handle DNF etc

            let nr = strip_tags(cells[interestingcolumns[1]].to_string());
            athlete.bib = Some(nr.parse().unwrap());

            let gender = strip_tags(cells[interestingcolumns[2]].to_string());
            if gender == "F" {
                athlete.gender = athlete::Gender::Female;
            }

            athlete.name = strip_tags(cells[interestingcolumns[3]].to_string());

            let mut guntime = strip_tags(cells[interestingcolumns[5]].to_string());
            if guntime.chars().filter(|&c| c == ':').count() == 1 {
                let time = guntime;
                guntime = "0:".to_string();
                guntime.push_str(&time);
            }
            athlete.guntime = guntime;

            let location = strip_tags(cells[interestingcolumns[6]].to_string());
            let location = location.trim();
            if location != "" {
                athlete.location = Some(location.to_string());
            }

            // println!("Athlete: {:?}", athlete);
            athletes.push(athlete);
        }
    }

    athletes
}

fn strip_tags(taggedstr: String) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for chr in taggedstr.chars() {
        if !in_tag {
            match chr {
                '<' => in_tag = true,
                _ => result.push(chr),
            };
        }
        if in_tag {
            // like a match where we only care about this possibility
            if let '>' = chr {
                in_tag = false
            }
        }
    }
    result.trim().to_string()
}

/// Given a chronorace source string, gets the urls for the following pages of the results.
pub fn parse_page_urls(content: &str) -> Vec<String> {
    let startidx = match content.find("<b>Page: </b>") {
        Some(idx) => idx,
        None => 0,
    };
    let (_, afterpage) = content.split_at(startidx);
    let newlineidx = match afterpage.find('\n') {
        Some(idx) => idx,
        None => 0,
    };
    let (linksstr, _) = afterpage.split_at(newlineidx);

    if !linksstr.contains("a href") {
        return Vec::new();
    }

    let mut results: Vec<String> = Vec::new();
    for piece in linksstr.split('"') {
        if piece.contains("classement") {
            let mut url = String::new();
            url.push_str("http://www.chronorace.be/Classements/");
            url.push_str(piece);
            results.push(url);
        }
    }

    results
}

#[test]
fn test_parse_page_urls() {
    let content = include_str!("test-chronorace.html").to_string();
    let urls = parse_page_urls(&content);
    assert_eq!(urls.len(), 5);
    assert_eq!(urls[0],
               "http://www.chronorace.be/Classements/classement.\
                aspx?eventId=1186557729972765&mode=large&IdClassement=13026&srch=&scope=All&page=1");
    assert_eq!(urls[1],
               "http://www.chronorace.be/Classements/classement.\
                aspx?eventId=1186557729972765&mode=large&IdClassement=13026&srch=&scope=All&page=2");
    assert_eq!(urls[2],
               "http://www.chronorace.be/Classements/classement.\
                aspx?eventId=1186557729972765&mode=large&IdClassement=13026&srch=&scope=All&page=3");
    assert_eq!(urls[3],
               "http://www.chronorace.be/Classements/classement.\
                aspx?eventId=1186557729972765&mode=large&IdClassement=13026&srch=&scope=All&page=4");
    assert_eq!(urls[4],
               "http://www.chronorace.be/Classements/classement.\
                aspx?eventId=1186557729972765&mode=large&IdClassement=13026&srch=&scope=All&page=5");
    // assert!(false);
}

#[test]
fn test_parse_athletes() {
    let content = include_str!("test-chronorace.html").to_string();
    let athletes = parse_athletes(&content);
    assert_eq!(athletes.len(), 500);
}

#[test]
fn test_dnf_parse_athletes() {
    let content = include_str!("./test-chronorace-with-dnf.html").to_string();
    let athletes = parse_athletes(&content);
    assert_eq!(athletes.len(), 298);
}

#[test]
fn test_row_highlights_parse_athletes() {
    let content = include_str!("./test-chronorace-row-highlights.html").to_string();
    let athletes = parse_athletes(&content);
    assert_eq!(athletes.len(), 500);
}
