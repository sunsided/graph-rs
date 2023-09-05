//! Neo4j's [Movies Graph Example](https://github.com/neo4j-graph-examples/movies).

#![allow(dead_code)]

use crate::Graph;

#[derive(Debug)]
pub enum NodeType {
    Movie(Movie),
    Person(Person),
}

#[derive(Debug)]
pub enum RelationType {
    /// Person acted in movie
    ActedIn(ActedIn),
    /// Person directed movie
    Directed,
    /// Person produced movie
    Produced,
    /// Person wrote movie
    Wrote,
    /// Person follows person
    Follows,
    /// Person reviewed movie
    Reviewed(Review),
}

#[derive(Debug)]
pub struct Person {
    name: String,
    born: u16,
}

#[derive(Debug)]
pub struct Movie {
    title: String,
    released: u16,
    tagline: String,
}

#[derive(Debug)]
pub struct ActedIn {
    roles: Vec<String>,
}

#[derive(Debug)]
pub struct Review {
    summary: String,
    rating: u8,
}

pub fn movie_graph() -> Graph<NodeType, RelationType> {
    let mut graph = Graph::default();

    // The Matrix
    let matrix = graph.add(NodeType::Movie(Movie {
        title: "The Matrix".into(),
        released: 1999,
        tagline: "Welcome to the Real World".into(),
    }));

    let keanu = graph.add(NodeType::Person(Person {
        name: "Keanu Reeves".into(),
        born: 1964,
    }));
    let carrie = graph.add(NodeType::Person(Person {
        name: "Carrie-Anne Moss".into(),
        born: 1967,
    }));
    let laurence = graph.add(NodeType::Person(Person {
        name: "Laurence Fishburne".into(),
        born: 1961,
    }));
    let hugo = graph.add(NodeType::Person(Person {
        name: "Hugo Weaving".into(),
        born: 1960,
    }));
    let lilly_w = graph.add(NodeType::Person(Person {
        name: "Lilly Wachowski".into(),
        born: 1967,
    }));
    let lana_w = graph.add(NodeType::Person(Person {
        name: "Lana Wachowski".into(),
        born: 1965,
    }));
    let joel_s = graph.add(NodeType::Person(Person {
        name: "Joel Silver".into(),
        born: 1952,
    }));
    let emil_e = graph.add(NodeType::Person(Person {
        name: "Emil Eifrem".into(),
        born: 1978,
    }));

    graph.link_to(
        &keanu,
        &matrix,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Neo".into()]),
        }),
    );
    graph.link_to(
        &carrie,
        &matrix,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Trinity".into()]),
        }),
    );
    graph.link_to(
        &laurence,
        &matrix,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Morpheus".into()]),
        }),
    );
    graph.link_to(
        &hugo,
        &matrix,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Agent Smith".into()]),
        }),
    );
    graph.link_to(
        &emil_e,
        &matrix,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Emil".into()]),
        }),
    );
    graph.link_to(&lilly_w, &matrix, RelationType::Directed);
    graph.link_to(&lana_w, &matrix, RelationType::Directed);
    graph.link_to(&joel_s, &matrix, RelationType::Produced);

    // The Matrix Reloaded
    let matrix_reloaded = graph.add(NodeType::Movie(Movie {
        title: "The Matrix Reloaded".into(),
        released: 2003,
        tagline: "Free your mind".into(),
    }));
    graph.link_to(
        &keanu,
        &matrix_reloaded,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Neo".into()]),
        }),
    );
    graph.link_to(
        &carrie,
        &matrix_reloaded,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Trinity".into()]),
        }),
    );
    graph.link_to(
        &laurence,
        &matrix_reloaded,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Morpheus".into()]),
        }),
    );
    graph.link_to(
        &hugo,
        &matrix_reloaded,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Agent Smith".into()]),
        }),
    );
    graph.link_to(&lilly_w, &matrix_reloaded, RelationType::Directed);
    graph.link_to(&lana_w, &matrix_reloaded, RelationType::Directed);
    graph.link_to(&joel_s, &matrix_reloaded, RelationType::Produced);

    // The Matrix Revolutions
    let matrix_revolutions = graph.add(NodeType::Movie(Movie {
        title: "The Matrix Revolutions".into(),
        released: 2003,
        tagline: "Everything that has a beginning has an end".into(),
    }));
    graph.link_to(
        &keanu,
        &matrix_revolutions,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Neo".into()]),
        }),
    );
    graph.link_to(
        &carrie,
        &matrix_revolutions,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Trinity".into()]),
        }),
    );
    graph.link_to(
        &laurence,
        &matrix_revolutions,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Morpheus".into()]),
        }),
    );
    graph.link_to(
        &hugo,
        &matrix_revolutions,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Agent Smith".into()]),
        }),
    );
    graph.link_to(&lilly_w, &matrix_revolutions, RelationType::Directed);
    graph.link_to(&lana_w, &matrix_revolutions, RelationType::Directed);
    graph.link_to(&joel_s, &matrix_revolutions, RelationType::Produced);

    // The Devil's Advocate
    let devils_advocate = graph.add(NodeType::Movie(Movie {
        title: "The Devil's Advocate".into(),
        released: 1997,
        tagline: "Evil has its winning ways".into(),
    }));
    let charlize = graph.add(NodeType::Person(Person {
        name: "Charlize Theron".into(),
        born: 1975,
    }));
    let al = graph.add(NodeType::Person(Person {
        name: "Al Pacino".into(),
        born: 1940,
    }));
    let taylor = graph.add(NodeType::Person(Person {
        name: "Taylor Hackford".into(),
        born: 1944,
    }));
    graph.link_to(
        &keanu,
        &devils_advocate,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Kevin Lomax".into()]),
        }),
    );
    graph.link_to(
        &charlize,
        &devils_advocate,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Mary Ann Lomax".into()]),
        }),
    );
    graph.link_to(
        &al,
        &devils_advocate,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["John Milton".into()]),
        }),
    );
    graph.link_to(&taylor, &devils_advocate, RelationType::Directed);

    // A Few Good Men
    let a_few_good_men = graph.add(NodeType::Movie(Movie {
        title: "A Few Good Men".into(),
        released: 1992,
        tagline: "In the heart of the nation's capital, in a courthouse of the U.S. government, one man will stop at nothing to keep his honor, and one will stop at nothing to find the truth.".into(),
    }));
    let tom_c = graph.add(NodeType::Person(Person {
        name: "Tom Cruise".into(),
        born: 1962,
    }));
    let jack_n = graph.add(NodeType::Person(Person {
        name: "Jack Nicholson".into(),
        born: 1937,
    }));
    let demi_m = graph.add(NodeType::Person(Person {
        name: "Demi Moore".into(),
        born: 1962,
    }));
    let kevin_b = graph.add(NodeType::Person(Person {
        name: "Kevin Bacon".into(),
        born: 1958,
    }));
    let kiefer_s = graph.add(NodeType::Person(Person {
        name: "Kiefer Sutherland".into(),
        born: 1966,
    }));
    let noah_w = graph.add(NodeType::Person(Person {
        name: "Noah Wyle".into(),
        born: 1971,
    }));
    let cuba_g = graph.add(NodeType::Person(Person {
        name: "Cuba Gooding Jr.".into(),
        born: 1968,
    }));
    let kevin_p = graph.add(NodeType::Person(Person {
        name: "Kevin Pollack".into(),
        born: 1968,
    }));
    let jtw = graph.add(NodeType::Person(Person {
        name: "J.T. Walsh".into(),
        born: 1943,
    }));
    let james_m = graph.add(NodeType::Person(Person {
        name: "James Marshall".into(),
        born: 1967,
    }));
    let christopher_g = graph.add(NodeType::Person(Person {
        name: "Christopher Guest".into(),
        born: 1948,
    }));
    let rob_r = graph.add(NodeType::Person(Person {
        name: "Rob Reiner".into(),
        born: 1947,
    }));
    let aaron_s = graph.add(NodeType::Person(Person {
        name: "Aaron Sorkin".into(),
        born: 1961,
    }));
    graph.link_to(
        &tom_c,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Lt. Daniel Kaffee".into()]),
        }),
    );
    graph.link_to(
        &jack_n,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Col. Nathan R. Jessup".into()]),
        }),
    );
    graph.link_to(
        &demi_m,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Lt. Cdr. JoAnne Galloway".into()]),
        }),
    );
    graph.link_to(
        &kevin_b,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Capt. Jack Ross".into()]),
        }),
    );
    graph.link_to(
        &kiefer_s,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Lt. Jonathan Kendrick".into()]),
        }),
    );
    graph.link_to(
        &noah_w,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Cpl. Jeffrey Barnes".into()]),
        }),
    );
    graph.link_to(
        &cuba_g,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Cpl. Carl Hammaker".into()]),
        }),
    );
    graph.link_to(
        &kevin_p,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Lt. Sam Weinberg".into()]),
        }),
    );
    graph.link_to(
        &jtw,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Lt. Col. Matthew Andrew Markinson".into()]),
        }),
    );
    graph.link_to(
        &james_m,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Pfc. Louden Downey".into()]),
        }),
    );
    graph.link_to(
        &christopher_g,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Dr. Stone".into()]),
        }),
    );
    graph.link_to(
        &aaron_s,
        &a_few_good_men,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Man in Bar".into()]),
        }),
    );
    graph.link_to(&rob_r, &matrix, RelationType::Directed);
    graph.link_to(&aaron_s, &matrix, RelationType::Wrote);

    // Top Gun
    let top_gun = graph.add(NodeType::Movie(Movie {
        title: "Top Gun".into(),
        released: 1986,
        tagline: "I feel the need, the need for speed".into(),
    }));
    let kelly_m = graph.add(NodeType::Person(Person {
        name: "Kelly McGillis".into(),
        born: 1957,
    }));
    let val_k = graph.add(NodeType::Person(Person {
        name: "Val Kilmer".into(),
        born: 1959,
    }));
    let anthony_e = graph.add(NodeType::Person(Person {
        name: "Anthony Edwards".into(),
        born: 1962,
    }));
    let tom_s = graph.add(NodeType::Person(Person {
        name: "Tom Skerritt".into(),
        born: 1933,
    }));
    let meg_r = graph.add(NodeType::Person(Person {
        name: "Meg Ryan".into(),
        born: 1961,
    }));
    let tony_s = graph.add(NodeType::Person(Person {
        name: "Tony Scott".into(),
        born: 1944,
    }));
    let jim_c = graph.add(NodeType::Person(Person {
        name: "Jim Cash".into(),
        born: 1941,
    }));
    graph.link_to(
        &tom_c,
        &top_gun,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Maverick".into()]),
        }),
    );
    graph.link_to(
        &kelly_m,
        &top_gun,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Charlie".into()]),
        }),
    );
    graph.link_to(
        &val_k,
        &top_gun,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Iceman".into()]),
        }),
    );
    graph.link_to(
        &anthony_e,
        &top_gun,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Goose".into()]),
        }),
    );
    graph.link_to(
        &tom_s,
        &top_gun,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Viper".into()]),
        }),
    );
    graph.link_to(
        &meg_r,
        &top_gun,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Carole".into()]),
        }),
    );
    graph.link_to(&tony_s, &top_gun, RelationType::Directed);
    graph.link_to(&jim_c, &top_gun, RelationType::Wrote);

    let jerry_maguire = graph.add(NodeType::Movie(Movie {
        title: "Jerry Maguire".into(),
        released: 2000,
        tagline: "The rest of his life begins now.".into(),
    }));
    let renee_z = graph.add(NodeType::Person(Person {
        name: "Renee Zellweger".into(),
        born: 1969,
    }));
    let kelly_p = graph.add(NodeType::Person(Person {
        name: "Kelly Preston".into(),
        born: 1962,
    }));
    let jerry_o = graph.add(NodeType::Person(Person {
        name: "Jerry O'Connell".into(),
        born: 1974,
    }));
    let jay_m = graph.add(NodeType::Person(Person {
        name: "Jay Mohr".into(),
        born: 1970,
    }));
    let bonnie_h = graph.add(NodeType::Person(Person {
        name: "Bonnie Hunt".into(),
        born: 1961,
    }));
    let regina_k = graph.add(NodeType::Person(Person {
        name: "Regina King".into(),
        born: 1971,
    }));
    let jonathan_l = graph.add(NodeType::Person(Person {
        name: "Jonathan Lipnicki".into(),
        born: 1996,
    }));
    let cameron_c = graph.add(NodeType::Person(Person {
        name: "Cameron Crowe".into(),
        born: 1957,
    }));
    graph.link_to(
        &tom_c,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jerry Maguire".into()]),
        }),
    );
    graph.link_to(
        &cuba_g,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Rod Tidwell".into()]),
        }),
    );
    graph.link_to(
        &renee_z,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Dorothy Boyd".into()]),
        }),
    );
    graph.link_to(
        &kelly_p,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Avery Bishop".into()]),
        }),
    );
    graph.link_to(
        &jerry_o,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Frank Cushman".into()]),
        }),
    );
    graph.link_to(
        &jay_m,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Bob Sugar".into()]),
        }),
    );
    graph.link_to(
        &bonnie_h,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Laurel Boyd".into()]),
        }),
    );
    graph.link_to(
        &regina_k,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Marcee Tidwell".into()]),
        }),
    );
    graph.link_to(
        &jonathan_l,
        &jerry_maguire,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Ray Boyd".into()]),
        }),
    );
    graph.link_to(&cameron_c, &jerry_maguire, RelationType::Directed);
    graph.link_to(&cameron_c, &jerry_maguire, RelationType::Produced);
    graph.link_to(&cameron_c, &jerry_maguire, RelationType::Wrote);

    let stand_by_me = graph.add(NodeType::Movie(Movie { title: "Stand By Me".into(), released: 1986, tagline: "For some, it's the last real taste of innocence, and the first real taste of life. But for everyone, it's the time that memories are made of.".into()}));
    let river_p = graph.add(NodeType::Person(Person {
        name: "River Phoenix".into(),
        born: 1970,
    }));
    let corey_f = graph.add(NodeType::Person(Person {
        name: "Corey Feldman".into(),
        born: 1971,
    }));
    let wil_w = graph.add(NodeType::Person(Person {
        name: "Wil Wheaton".into(),
        born: 1972,
    }));
    let john_c = graph.add(NodeType::Person(Person {
        name: "John Cusack".into(),
        born: 1966,
    }));
    let marshall_b = graph.add(NodeType::Person(Person {
        name: "Marshall Bell".into(),
        born: 1942,
    }));
    graph.link_to(
        &wil_w,
        &stand_by_me,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Gordie Lachance".into()]),
        }),
    );
    graph.link_to(
        &river_p,
        &stand_by_me,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Chris Chambers".into()]),
        }),
    );
    graph.link_to(
        &jerry_o,
        &stand_by_me,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Vern Tessio".into()]),
        }),
    );
    graph.link_to(
        &corey_f,
        &stand_by_me,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Teddy Duchamp".into()]),
        }),
    );
    graph.link_to(
        &john_c,
        &stand_by_me,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Denny Lachance".into()]),
        }),
    );
    graph.link_to(
        &kiefer_s,
        &stand_by_me,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Ace Merrill".into()]),
        }),
    );
    graph.link_to(
        &marshall_b,
        &stand_by_me,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Mr. Lachance".into()]),
        }),
    );
    graph.link_to(&rob_r, &stand_by_me, RelationType::Directed);

    let as_good_as_it_gets = graph.add(NodeType::Movie(Movie {
        title: "As Good as It Gets".into(),
        released: 1997,
        tagline: "A comedy from the heart that goes for the throat.".into(),
    }));
    let helen_h = graph.add(NodeType::Person(Person {
        name: "Helen Hunt".into(),
        born: 1963,
    }));
    let greg_k = graph.add(NodeType::Person(Person {
        name: "Greg Kinnear".into(),
        born: 1963,
    }));
    let james_b = graph.add(NodeType::Person(Person {
        name: "James L. Brooks".into(),
        born: 1940,
    }));
    graph.link_to(
        &jack_n,
        &as_good_as_it_gets,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Melvin Udall".into()]),
        }),
    );
    graph.link_to(
        &helen_h,
        &as_good_as_it_gets,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Carol Connelly".into()]),
        }),
    );
    graph.link_to(
        &greg_k,
        &as_good_as_it_gets,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Simon Bishop".into()]),
        }),
    );
    graph.link_to(
        &cuba_g,
        &as_good_as_it_gets,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Frank Sachs".into()]),
        }),
    );
    graph.link_to(&james_b, &as_good_as_it_gets, RelationType::Directed);

    let what_dreams_may_come = graph.add(NodeType::Movie(Movie {
        title: "What Dreams May Come".into(),
        released: 1998,
        tagline: "After life there is more. The end is just the beginning.".into(),
    }));
    let annabella_s = graph.add(NodeType::Person(Person {
        name: "Annabella Sciorra".into(),
        born: 1960,
    }));
    let max_s = graph.add(NodeType::Person(Person {
        name: "Max von Sydow".into(),
        born: 1929,
    }));
    let werner_h = graph.add(NodeType::Person(Person {
        name: "Werner Herzog".into(),
        born: 1942,
    }));
    let robin = graph.add(NodeType::Person(Person {
        name: "Robin Williams".into(),
        born: 1951,
    }));
    let vincent_w = graph.add(NodeType::Person(Person {
        name: "Vincent Ward".into(),
        born: 1956,
    }));
    graph.link_to(
        &robin,
        &what_dreams_may_come,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Chris Nielsen".into()]),
        }),
    );
    graph.link_to(
        &cuba_g,
        &what_dreams_may_come,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Albert Lewis".into()]),
        }),
    );
    graph.link_to(
        &annabella_s,
        &what_dreams_may_come,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Annie Collins-Nielsen".into()]),
        }),
    );
    graph.link_to(
        &max_s,
        &what_dreams_may_come,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["The Tracker".into()]),
        }),
    );
    graph.link_to(
        &werner_h,
        &what_dreams_may_come,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["The Face".into()]),
        }),
    );
    graph.link_to(&vincent_w, &what_dreams_may_come, RelationType::Directed);

    let snow_fallingon_cedars = graph.add(NodeType::Movie(Movie {
        title: "Snow Falling on Cedars".into(),
        released: 1999,
        tagline: "First loves last. Forever.".into(),
    }));
    let ethan_h = graph.add(NodeType::Person(Person {
        name: "Ethan Hawke".into(),
        born: 1970,
    }));
    let rick_y = graph.add(NodeType::Person(Person {
        name: "Rick Yune".into(),
        born: 1971,
    }));
    let james_c = graph.add(NodeType::Person(Person {
        name: "James Cromwell".into(),
        born: 1940,
    }));
    let scott_h = graph.add(NodeType::Person(Person {
        name: "Scott Hicks".into(),
        born: 1953,
    }));
    graph.link_to(
        &ethan_h,
        &snow_fallingon_cedars,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Ishmael Chambers".into()]),
        }),
    );
    graph.link_to(
        &rick_y,
        &snow_fallingon_cedars,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Kazuo Miyamoto".into()]),
        }),
    );
    graph.link_to(
        &max_s,
        &snow_fallingon_cedars,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Nels Gudmundsson".into()]),
        }),
    );
    graph.link_to(
        &james_c,
        &snow_fallingon_cedars,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Judge Fielding".into()]),
        }),
    );
    graph.link_to(&scott_h, &snow_fallingon_cedars, RelationType::Directed);

    let youve_got_mail = graph.add(NodeType::Movie(Movie {
        title: "You've Got Mail".into(),
        released: 1998,
        tagline: "At odds in life... in love on-line.".into(),
    }));
    let parker_p = graph.add(NodeType::Person(Person {
        name: "Parker Posey".into(),
        born: 1968,
    }));
    let dave_c = graph.add(NodeType::Person(Person {
        name: "Dave Chappelle".into(),
        born: 1973,
    }));
    let steve_z = graph.add(NodeType::Person(Person {
        name: "Steve Zahn".into(),
        born: 1967,
    }));
    let tom_h = graph.add(NodeType::Person(Person {
        name: "Tom Hanks".into(),
        born: 1956,
    }));
    let nora_e = graph.add(NodeType::Person(Person {
        name: "Nora Ephron".into(),
        born: 1941,
    }));
    graph.link_to(
        &tom_h,
        &youve_got_mail,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Joe Fox".into()]),
        }),
    );
    graph.link_to(
        &meg_r,
        &youve_got_mail,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Kathleen Kelly".into()]),
        }),
    );
    graph.link_to(
        &greg_k,
        &youve_got_mail,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Frank Navasky".into()]),
        }),
    );
    graph.link_to(
        &parker_p,
        &youve_got_mail,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Patricia Eden".into()]),
        }),
    );
    graph.link_to(
        &dave_c,
        &youve_got_mail,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Kevin Jackson".into()]),
        }),
    );
    graph.link_to(
        &steve_z,
        &youve_got_mail,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["George Pappas".into()]),
        }),
    );
    graph.link_to(&nora_e, &youve_got_mail, RelationType::Directed);

    let sleepless_in_seattle = graph.add(NodeType::Movie(Movie { title: "Sleepless in Seattle".into(), released: 1993, tagline: "What if someone you never met, someone you never saw, someone you never knew was the only someone for you?".into()}));
    let rita_w = graph.add(NodeType::Person(Person {
        name: "Rita Wilson".into(),
        born: 1956,
    }));
    let bill_pull = graph.add(NodeType::Person(Person {
        name: "Bill Pullman".into(),
        born: 1953,
    }));
    let victor_g = graph.add(NodeType::Person(Person {
        name: "Victor Garber".into(),
        born: 1949,
    }));
    let rosie_o = graph.add(NodeType::Person(Person {
        name: "Rosie O'Donnell".into(),
        born: 1962,
    }));
    graph.link_to(
        &tom_h,
        &sleepless_in_seattle,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Sam Baldwin".into()]),
        }),
    );
    graph.link_to(
        &meg_r,
        &sleepless_in_seattle,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Annie Reed".into()]),
        }),
    );
    graph.link_to(
        &rita_w,
        &sleepless_in_seattle,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Suzy".into()]),
        }),
    );
    graph.link_to(
        &bill_pull,
        &sleepless_in_seattle,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Walter".into()]),
        }),
    );
    graph.link_to(
        &victor_g,
        &sleepless_in_seattle,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Greg".into()]),
        }),
    );
    graph.link_to(
        &rosie_o,
        &sleepless_in_seattle,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Becky".into()]),
        }),
    );
    graph.link_to(&nora_e, &sleepless_in_seattle, RelationType::Directed);

    let joe_versusthe_volcano = graph.add(NodeType::Movie(Movie {
        title: "Joe Versus the Volcano".into(),
        released: 1990,
        tagline: "A story of love, lava and burning desire.".into(),
    }));
    let john_s = graph.add(NodeType::Person(Person {
        name: "John Patrick Stanley".into(),
        born: 1950,
    }));
    let nathan = graph.add(NodeType::Person(Person {
        name: "Nathan Lane".into(),
        born: 1956,
    }));
    graph.link_to(
        &tom_h,
        &joe_versusthe_volcano,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Joe Banks".into()]),
        }),
    );
    graph.link_to(
        &meg_r,
        &joe_versusthe_volcano,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["DeDe', 'Angelica Graynamore', 'Patricia Graynamore".into()]),
        }),
    );
    graph.link_to(
        &nathan,
        &joe_versusthe_volcano,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Baw".into()]),
        }),
    );
    graph.link_to(&john_s, &joe_versusthe_volcano, RelationType::Directed);

    let when_harry_met_sally = graph.add(NodeType::Movie(Movie {
        title: "When Harry Met Sally".into(),
        released: 1998,
        tagline: "Can two friends sleep together and still love each other in the morning?".into(),
    }));
    let billy_c = graph.add(NodeType::Person(Person {
        name: "Billy Crystal".into(),
        born: 1948,
    }));
    let carrie_f = graph.add(NodeType::Person(Person {
        name: "Carrie Fisher".into(),
        born: 1956,
    }));
    let bruno_k = graph.add(NodeType::Person(Person {
        name: "Bruno Kirby".into(),
        born: 1949,
    }));
    graph.link_to(
        &billy_c,
        &when_harry_met_sally,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Harry Burns".into()]),
        }),
    );
    graph.link_to(
        &meg_r,
        &when_harry_met_sally,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Sally Albright".into()]),
        }),
    );
    graph.link_to(
        &carrie_f,
        &when_harry_met_sally,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Marie".into()]),
        }),
    );
    graph.link_to(
        &bruno_k,
        &when_harry_met_sally,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jess".into()]),
        }),
    );
    graph.link_to(&rob_r, &when_harry_met_sally, RelationType::Directed);
    graph.link_to(&rob_r, &when_harry_met_sally, RelationType::Produced);
    graph.link_to(&nora_e, &when_harry_met_sally, RelationType::Produced);
    graph.link_to(&nora_e, &when_harry_met_sally, RelationType::Wrote);

    let that_thing_you_do = graph.add(NodeType::Movie(Movie {
        title: "That Thing You Do".into(),
        released: 1996,
        tagline:
            "In every life there comes a time when that thing you dream becomes that thing you do"
                .into(),
    }));
    let liv_t = graph.add(NodeType::Person(Person {
        name: "Liv Tyler".into(),
        born: 1977,
    }));
    graph.link_to(
        &tom_h,
        &that_thing_you_do,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Mr. White".into()]),
        }),
    );
    graph.link_to(
        &liv_t,
        &that_thing_you_do,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Faye Dolan".into()]),
        }),
    );
    graph.link_to(
        &charlize,
        &that_thing_you_do,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Tina".into()]),
        }),
    );
    graph.link_to(&tom_h, &that_thing_you_do, RelationType::Directed);

    let the_replacements = graph.add(NodeType::Movie(Movie {
        title: "The Replacements".into(),
        released: 2000,
        tagline: "Pain heals, Chicks dig scars... Glory lasts forever".into(),
    }));
    let brooke = graph.add(NodeType::Person(Person {
        name: "Brooke Langton".into(),
        born: 1970,
    }));
    let gene = graph.add(NodeType::Person(Person {
        name: "Gene Hackman".into(),
        born: 1930,
    }));
    let orlando = graph.add(NodeType::Person(Person {
        name: "Orlando Jones".into(),
        born: 1968,
    }));
    let howard = graph.add(NodeType::Person(Person {
        name: "Howard Deutch".into(),
        born: 1950,
    }));
    graph.link_to(
        &keanu,
        &the_replacements,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Shane Falco".into()]),
        }),
    );
    graph.link_to(
        &brooke,
        &the_replacements,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Annabelle Farrell".into()]),
        }),
    );
    graph.link_to(
        &gene,
        &the_replacements,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jimmy McGinty".into()]),
        }),
    );
    graph.link_to(
        &orlando,
        &the_replacements,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Clifford Franklin".into()]),
        }),
    );
    graph.link_to(&howard, &the_replacements, RelationType::Directed);

    let rescue_dawn = graph.add(NodeType::Movie(Movie {
        title: "RescueDawn".into(),
        released: 2006,
        tagline: "Based on the extraordinary true story of one man's fight for freedom".into(),
    }));
    let christian_b = graph.add(NodeType::Person(Person {
        name: "Christian Bale".into(),
        born: 1974,
    }));
    let zach_g = graph.add(NodeType::Person(Person {
        name: "Zach Grenier".into(),
        born: 1954,
    }));
    graph.link_to(
        &marshall_b,
        &rescue_dawn,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Admiral".into()]),
        }),
    );
    graph.link_to(
        &christian_b,
        &rescue_dawn,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Dieter Dengler".into()]),
        }),
    );
    graph.link_to(
        &zach_g,
        &rescue_dawn,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Squad Leader".into()]),
        }),
    );
    graph.link_to(
        &steve_z,
        &rescue_dawn,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Duane".into()]),
        }),
    );
    graph.link_to(&werner_h, &rescue_dawn, RelationType::Directed);

    let the_birdcage = graph.add(NodeType::Movie(Movie {
        title: "The Birdcage".into(),
        released: 1996,
        tagline: "Come as you are".into(),
    }));
    let mike_n = graph.add(NodeType::Person(Person {
        name: "Mike Nichols".into(),
        born: 1931,
    }));
    graph.link_to(
        &robin,
        &the_birdcage,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Armand Goldman".into()]),
        }),
    );
    graph.link_to(
        &nathan,
        &the_birdcage,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Albert Goldman".into()]),
        }),
    );
    graph.link_to(
        &gene,
        &the_birdcage,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Sen. Kevin Keeley".into()]),
        }),
    );
    graph.link_to(&mike_n, &the_birdcage, RelationType::Directed);

    let unforgiven = graph.add(NodeType::Movie(Movie {
        title: "unforgiven".into(),
        released: 1992,
        tagline: "It's a hell of a thing, killing a man".into(),
    }));
    let richard_h = graph.add(NodeType::Person(Person {
        name: "Richard Harris".into(),
        born: 1930,
    }));
    let clint_e = graph.add(NodeType::Person(Person {
        name: "Clint Eastwood".into(),
        born: 1930,
    }));
    graph.link_to(
        &richard_h,
        &unforgiven,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["English Bob".into()]),
        }),
    );
    graph.link_to(
        &clint_e,
        &unforgiven,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Bill Munny".into()]),
        }),
    );
    graph.link_to(
        &gene,
        &unforgiven,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Little Bill Daggett".into()]),
        }),
    );
    graph.link_to(&clint_e, &unforgiven, RelationType::Directed);

    let johnny_mnemonic = graph.add(NodeType::Movie(Movie {
        title: "Johnny Mnemonic".into(),
        released: 1995,
        tagline: "The hottest data on earth. In the coolest head in town".into(),
    }));
    let takeshi = graph.add(NodeType::Person(Person {
        name: "Takeshi Kitano".into(),
        born: 1947,
    }));
    let dina = graph.add(NodeType::Person(Person {
        name: "Dina Meyer".into(),
        born: 1968,
    }));
    let ice_t = graph.add(NodeType::Person(Person {
        name: "Ice-T".into(),
        born: 1958,
    }));
    let robert_l = graph.add(NodeType::Person(Person {
        name: "Robert Longo".into(),
        born: 1953,
    }));
    graph.link_to(
        &keanu,
        &johnny_mnemonic,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Johnny Mnemonic".into()]),
        }),
    );
    graph.link_to(
        &takeshi,
        &johnny_mnemonic,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Takahashi".into()]),
        }),
    );
    graph.link_to(
        &dina,
        &johnny_mnemonic,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jane".into()]),
        }),
    );
    graph.link_to(
        &ice_t,
        &johnny_mnemonic,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["J-Bone".into()]),
        }),
    );
    graph.link_to(&robert_l, &johnny_mnemonic, RelationType::Directed);

    let cloud_atlas = graph.add(NodeType::Movie(Movie {
        title: "Cloud Atlas".into(),
        released: 2012,
        tagline: "Everything is connected".into(),
    }));
    let halle_b = graph.add(NodeType::Person(Person {
        name: "Halle Berry".into(),
        born: 1966,
    }));
    let jim_b = graph.add(NodeType::Person(Person {
        name: "Jim Broadbent".into(),
        born: 1949,
    }));
    let tom_t = graph.add(NodeType::Person(Person {
        name: "Tom Tykwer".into(),
        born: 1965,
    }));
    let david_mitchell = graph.add(NodeType::Person(Person {
        name: "David Mitchell".into(),
        born: 1969,
    }));
    let stefan_arndt = graph.add(NodeType::Person(Person {
        name: "Stefan Arndt".into(),
        born: 1961,
    }));
    graph.link_to(
        &tom_h,
        &cloud_atlas,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter([
                "Zachry', 'Dr. Henry Goose', 'Isaac Sachs', 'Dermot Hoggins".into()
            ]),
        }),
    );
    graph.link_to(&hugo, &cloud_atlas, RelationType::ActedIn(ActedIn { roles: Vec::from_iter(["Bill Smoke', 'Haskell Moore', 'Tadeusz Kesselring', 'Nurse Noakes', 'Boardman Mephi', 'Old Georgie".into()]) }) );
    graph.link_to(
        &halle_b,
        &cloud_atlas,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Luisa Rey', 'Jocasta Ayrs', 'Ovid', 'Meronym".into()]),
        }),
    );
    graph.link_to(
        &jim_b,
        &cloud_atlas,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Vyvyan Ayrs', 'Captain Molyneux', 'Timothy Cavendish".into()]),
        }),
    );
    graph.link_to(&tom_t, &cloud_atlas, RelationType::Directed);
    graph.link_to(&lilly_w, &cloud_atlas, RelationType::Directed);
    graph.link_to(&lana_w, &cloud_atlas, RelationType::Directed);
    graph.link_to(&david_mitchell, &cloud_atlas, RelationType::Wrote);
    graph.link_to(&stefan_arndt, &cloud_atlas, RelationType::Produced);

    let the_da_vinci_code = graph.add(NodeType::Movie(Movie {
        title: "The Da Vinci Code".into(),
        released: 2006,
        tagline: "Break The Codes".into(),
    }));
    let ian_m = graph.add(NodeType::Person(Person {
        name: "Ian McKellen".into(),
        born: 1939,
    }));
    let audrey_t = graph.add(NodeType::Person(Person {
        name: "Audrey Tautou".into(),
        born: 1976,
    }));
    let paul_b = graph.add(NodeType::Person(Person {
        name: "Paul Bettany".into(),
        born: 1971,
    }));
    let ron_h = graph.add(NodeType::Person(Person {
        name: "Ron Howard".into(),
        born: 1954,
    }));
    graph.link_to(
        &tom_h,
        &the_da_vinci_code,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Dr. Robert Langdon".into()]),
        }),
    );
    graph.link_to(
        &ian_m,
        &the_da_vinci_code,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Sir Leight Teabing".into()]),
        }),
    );
    graph.link_to(
        &audrey_t,
        &the_da_vinci_code,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Sophie Neveu".into()]),
        }),
    );
    graph.link_to(
        &paul_b,
        &the_da_vinci_code,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Silas".into()]),
        }),
    );
    graph.link_to(&ron_h, &the_da_vinci_code, RelationType::Directed);

    let vfor_vendetta = graph.add(NodeType::Movie(Movie {
        title: "V for Vendetta".into(),
        released: 2006,
        tagline: "Freedom! Forever!".into(),
    }));
    let ben_m = graph.add(NodeType::Person(Person {
        name: "Ben Miles".into(),
        born: 1967,
    }));
    let natalie_p = graph.add(NodeType::Person(Person {
        name: "Natalie Portman".into(),
        born: 1981,
    }));
    let stephen_r = graph.add(NodeType::Person(Person {
        name: "Stephen Rea".into(),
        born: 1946,
    }));
    let john_h = graph.add(NodeType::Person(Person {
        name: "John Hurt".into(),
        born: 1940,
    }));
    graph.link_to(
        &hugo,
        &vfor_vendetta,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["V".into()]),
        }),
    );
    graph.link_to(
        &natalie_p,
        &vfor_vendetta,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Evey Hammond".into()]),
        }),
    );
    graph.link_to(
        &stephen_r,
        &vfor_vendetta,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Eric Finch".into()]),
        }),
    );
    graph.link_to(
        &john_h,
        &vfor_vendetta,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["High Chancellor Adam Sutler".into()]),
        }),
    );
    graph.link_to(
        &ben_m,
        &vfor_vendetta,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Dascomb".into()]),
        }),
    );
    graph.link_to(&james_m, &vfor_vendetta, RelationType::Directed);
    graph.link_to(&lilly_w, &vfor_vendetta, RelationType::Produced);
    graph.link_to(&lana_w, &vfor_vendetta, RelationType::Produced);
    graph.link_to(&joel_s, &vfor_vendetta, RelationType::Produced);
    graph.link_to(&lilly_w, &vfor_vendetta, RelationType::Wrote);
    graph.link_to(&lana_w, &vfor_vendetta, RelationType::Wrote);

    let speed_racer = graph.add(NodeType::Movie(Movie {
        title: "Speed Racer".into(),
        released: 2008,
        tagline: "Speed has no limits".into(),
    }));
    let emile_h = graph.add(NodeType::Person(Person {
        name: "Emile Hirsch".into(),
        born: 1985,
    }));
    let john_g = graph.add(NodeType::Person(Person {
        name: "John Goodman".into(),
        born: 1960,
    }));
    let susan_s = graph.add(NodeType::Person(Person {
        name: "Susan Sarandon".into(),
        born: 1946,
    }));
    let matthew_f = graph.add(NodeType::Person(Person {
        name: "Matthew Fox".into(),
        born: 1966,
    }));
    let christina_r = graph.add(NodeType::Person(Person {
        name: "Christina Ricci".into(),
        born: 1980,
    }));
    let rain = graph.add(NodeType::Person(Person {
        name: "Rain".into(),
        born: 1982,
    }));
    graph.link_to(
        &emile_h,
        &speed_racer,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Speed Racer".into()]),
        }),
    );
    graph.link_to(
        &john_g,
        &speed_racer,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Pops".into()]),
        }),
    );
    graph.link_to(
        &susan_s,
        &speed_racer,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Mom".into()]),
        }),
    );
    graph.link_to(
        &matthew_f,
        &speed_racer,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Racer X".into()]),
        }),
    );
    graph.link_to(
        &christina_r,
        &speed_racer,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Trixie".into()]),
        }),
    );
    graph.link_to(
        &rain,
        &speed_racer,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Taejo Togokahn".into()]),
        }),
    );
    graph.link_to(
        &ben_m,
        &speed_racer,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Cass Jones".into()]),
        }),
    );
    graph.link_to(&lilly_w, &speed_racer, RelationType::Directed);
    graph.link_to(&lana_w, &speed_racer, RelationType::Directed);
    graph.link_to(&lilly_w, &speed_racer, RelationType::Wrote);
    graph.link_to(&lana_w, &speed_racer, RelationType::Wrote);
    graph.link_to(&joel_s, &speed_racer, RelationType::Produced);

    let ninja_assassin = graph.add(NodeType::Movie(Movie {
        title: "Ninja Assassin".into(),
        released: 2009,
        tagline: "Prepare to enter a secret world of assassins".into(),
    }));
    let naomie_h = graph.add(NodeType::Person(Person {
        name: "Naomie Harris".into(),
        born: 0,
    }));
    graph.link_to(
        &rain,
        &ninja_assassin,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Raizo".into()]),
        }),
    );
    graph.link_to(
        &naomie_h,
        &ninja_assassin,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Mika Coretti".into()]),
        }),
    );
    graph.link_to(
        &rick_y,
        &ninja_assassin,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Takeshi".into()]),
        }),
    );
    graph.link_to(
        &ben_m,
        &ninja_assassin,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Ryan Maslow".into()]),
        }),
    );
    graph.link_to(&james_m, &ninja_assassin, RelationType::Directed);
    graph.link_to(&lilly_w, &ninja_assassin, RelationType::Produced);
    graph.link_to(&lana_w, &ninja_assassin, RelationType::Produced);
    graph.link_to(&joel_s, &ninja_assassin, RelationType::Produced);

    let the_green_mile = graph.add(NodeType::Movie(Movie {
        title: "The Green Mile".into(),
        released: 1999,
        tagline: "Walk a mile you'll never forget.".into(),
    }));
    let michael_d = graph.add(NodeType::Person(Person {
        name: "'Michael Clarke Duncan'".into(),
        born: 1957,
    }));
    let david_m = graph.add(NodeType::Person(Person {
        name: "'David Morse'".into(),
        born: 1953,
    }));
    let sam_r = graph.add(NodeType::Person(Person {
        name: "'Sam Rockwell'".into(),
        born: 1968,
    }));
    let gary_s = graph.add(NodeType::Person(Person {
        name: "'Gary Sinise'".into(),
        born: 1955,
    }));
    let patricia_c = graph.add(NodeType::Person(Person {
        name: "'Patricia Clarkson'".into(),
        born: 1959,
    }));
    let frank_d = graph.add(NodeType::Person(Person {
        name: "'Frank Darabont'".into(),
        born: 1959,
    }));
    graph.link_to(
        &tom_h,
        &the_green_mile,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Paul Edgecomb".into()]),
        }),
    );
    graph.link_to(
        &michael_d,
        &the_green_mile,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["John Coffey".into()]),
        }),
    );
    graph.link_to(
        &david_m,
        &the_green_mile,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Brutus \"Brutal\" Howell".into()]),
        }),
    );
    graph.link_to(
        &bonnie_h,
        &the_green_mile,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jan Edgecomb".into()]),
        }),
    );
    graph.link_to(
        &james_c,
        &the_green_mile,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Warden Hal Moores".into()]),
        }),
    );
    graph.link_to(
        &sam_r,
        &the_green_mile,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["\"Wild Bill\" Wharton".into()]),
        }),
    );
    graph.link_to(
        &gary_s,
        &the_green_mile,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Burt Hammersmith".into()]),
        }),
    );
    graph.link_to(
        &patricia_c,
        &the_green_mile,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Melinda Moores".into()]),
        }),
    );
    graph.link_to(&frank_d, &the_green_mile, RelationType::Directed);

    let frost_nixon = graph.add(NodeType::Movie(Movie {
        title: "Frost/Nixon".into(),
        released: 2008,
        tagline: "400 million people were waiting for the truth.".into(),
    }));
    let frank_l = graph.add(NodeType::Person(Person {
        name: "Frank Langella".into(),
        born: 1938,
    }));
    let michael_s = graph.add(NodeType::Person(Person {
        name: "Michael Sheen".into(),
        born: 1969,
    }));
    let oliver_p = graph.add(NodeType::Person(Person {
        name: "Oliver Platt".into(),
        born: 1960,
    }));
    graph.link_to(
        &frank_l,
        &frost_nixon,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Richard Nixon".into()]),
        }),
    );
    graph.link_to(
        &michael_s,
        &frost_nixon,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["David Frost".into()]),
        }),
    );
    graph.link_to(
        &kevin_b,
        &frost_nixon,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jack Brennan".into()]),
        }),
    );
    graph.link_to(
        &oliver_p,
        &frost_nixon,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Bob Zelnick".into()]),
        }),
    );
    graph.link_to(
        &sam_r,
        &frost_nixon,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["James Reston, Jr.".into()]),
        }),
    );
    graph.link_to(&ron_h, &frost_nixon, RelationType::Directed);

    let hoffa = graph.add(NodeType::Movie(Movie {
        title: "Hoffa".into(),
        released: 1992,
        tagline: "He didn't want law. He wanted justice.".into(),
    }));
    let danny_d = graph.add(NodeType::Person(Person {
        name: "Danny DeVito".into(),
        born: 1944,
    }));
    let john_r = graph.add(NodeType::Person(Person {
        name: "John C. Reilly".into(),
        born: 1965,
    }));
    graph.link_to(
        &jack_n,
        &hoffa,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Hoffa".into()]),
        }),
    );
    graph.link_to(
        &danny_d,
        &hoffa,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Robert \"Bobby\" Ciaro".into()]),
        }),
    );
    graph.link_to(
        &jtw,
        &hoffa,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Frank Fitzsimmons".into()]),
        }),
    );
    graph.link_to(
        &john_r,
        &hoffa,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Peter \"Pete\" Connelly".into()]),
        }),
    );
    graph.link_to(&danny_d, &hoffa, RelationType::Directed);

    let apollo13 = graph.add(NodeType::Movie(Movie {
        title: "Apollo 13".into(),
        released: 1995,
        tagline: "Houston, we have a problem.".into(),
    }));
    let ed_h = graph.add(NodeType::Person(Person {
        name: "Ed Harris".into(),
        born: 1950,
    }));
    let bill_pax = graph.add(NodeType::Person(Person {
        name: "Bill Paxton".into(),
        born: 1955,
    }));
    graph.link_to(
        &tom_h,
        &apollo13,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jim Lovell".into()]),
        }),
    );
    graph.link_to(
        &kevin_b,
        &apollo13,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jack Swigert".into()]),
        }),
    );
    graph.link_to(
        &ed_h,
        &apollo13,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Gene Kranz".into()]),
        }),
    );
    graph.link_to(
        &bill_pax,
        &apollo13,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Fred Haise".into()]),
        }),
    );
    graph.link_to(
        &gary_s,
        &apollo13,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Ken Mattingly".into()]),
        }),
    );
    graph.link_to(&ron_h, &apollo13, RelationType::Directed);

    let twister = graph.add(NodeType::Movie(Movie {
        title: "Twister".into(),
        released: 1996,
        tagline: "Don't Breathe. Don't Look Back.".into(),
    }));
    let philip_h = graph.add(NodeType::Person(Person {
        name: "Philip Seymour Hoffman".into(),
        born: 1967,
    }));
    let jan_b = graph.add(NodeType::Person(Person {
        name: "Jan de Bont".into(),
        born: 1943,
    }));
    graph.link_to(
        &bill_pax,
        &twister,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Bill Harding".into()]),
        }),
    );
    graph.link_to(
        &helen_h,
        &twister,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Dr. Jo Harding".into()]),
        }),
    );
    graph.link_to(
        &zach_g,
        &twister,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Eddie".into()]),
        }),
    );
    graph.link_to(
        &philip_h,
        &twister,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Dustin \"Dusty\" Davis".into()]),
        }),
    );
    graph.link_to(&jan_b, &twister, RelationType::Directed);

    let cast_away = graph.add(NodeType::Movie(Movie {
        title: "Cast Away".into(),
        released: 2000,
        tagline: "At the edge of the world, his journey begins.".into(),
    }));
    let robert_z = graph.add(NodeType::Person(Person {
        name: "Robert Zemeckis".into(),
        born: 1951,
    }));
    graph.link_to(
        &tom_h,
        &cast_away,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Chuck Noland".into()]),
        }),
    );
    graph.link_to(
        &helen_h,
        &cast_away,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Kelly Frears".into()]),
        }),
    );
    graph.link_to(&robert_z, &cast_away, RelationType::Directed);

    let one_flew_overthe_cuckoos_nest = graph.add(NodeType::Movie(Movie {
        title: "One Flew Over the Cuckoo's Nest".into(),
        released: 1975,
        tagline: "If he's crazy, what does that make you?".into(),
    }));
    let milos_f = graph.add(NodeType::Person(Person {
        name: "Milos Forman".into(),
        born: 1932,
    }));
    graph.link_to(
        &jack_n,
        &one_flew_overthe_cuckoos_nest,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Randle McMurphy".into()]),
        }),
    );
    graph.link_to(
        &danny_d,
        &one_flew_overthe_cuckoos_nest,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Martini".into()]),
        }),
    );
    graph.link_to(
        &milos_f,
        &one_flew_overthe_cuckoos_nest,
        RelationType::Directed,
    );

    let somethings_gotta_give = graph.add(NodeType::Movie(Movie {
        title: "Something's Gotta Give".into(),
        released: 2003,
        tagline: String::default(),
    }));
    let diane_k = graph.add(NodeType::Person(Person {
        name: "Diane Keaton".into(),
        born: 1946,
    }));
    let nancy_m = graph.add(NodeType::Person(Person {
        name: "Nancy Meyers".into(),
        born: 1949,
    }));
    graph.link_to(
        &jack_n,
        &somethings_gotta_give,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Harry Sanborn".into()]),
        }),
    );
    graph.link_to(
        &diane_k,
        &somethings_gotta_give,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Erica Barry".into()]),
        }),
    );
    graph.link_to(
        &keanu,
        &somethings_gotta_give,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Julian Mercer".into()]),
        }),
    );
    graph.link_to(&nancy_m, &somethings_gotta_give, RelationType::Directed);
    graph.link_to(&nancy_m, &somethings_gotta_give, RelationType::Produced);
    graph.link_to(&nancy_m, &somethings_gotta_give, RelationType::Wrote);

    let bicentennial_man = graph.add(NodeType::Movie(Movie {
        title: "Bicentennial Man".into(),
        released: 1999,
        tagline: "One robot's 200 year journey to become an ordinary man.".into(),
    }));
    let chris_c = graph.add(NodeType::Person(Person {
        name: "Chris Columbus".into(),
        born: 1958,
    }));
    graph.link_to(
        &robin,
        &bicentennial_man,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Andrew Marin".into()]),
        }),
    );
    graph.link_to(
        &oliver_p,
        &bicentennial_man,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Rupert Burns".into()]),
        }),
    );
    graph.link_to(&chris_c, &bicentennial_man, RelationType::Directed);

    let charlie_wilsons_war = graph.add(NodeType::Movie(Movie { title: "Charlie Wilson's War".into(), released: 2007, tagline: "A stiff drink. A little mascara. A lot of nerve. Who said they couldn't bring down the Soviet empire.".into()}));
    let julia_r = graph.add(NodeType::Person(Person {
        name: "Julia Roberts".into(),
        born: 1967,
    }));
    graph.link_to(
        &tom_h,
        &charlie_wilsons_war,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Rep. Charlie Wilson".into()]),
        }),
    );
    graph.link_to(
        &julia_r,
        &charlie_wilsons_war,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Joanne Herring".into()]),
        }),
    );
    graph.link_to(
        &philip_h,
        &charlie_wilsons_war,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Gust Avrakotos".into()]),
        }),
    );
    graph.link_to(&mike_n, &charlie_wilsons_war, RelationType::Directed);

    let the_polar_express = graph.add(NodeType::Movie(Movie {
        title: "The Polar Express".into(),
        released: 2004,
        tagline: "This Holiday Season… Believe".into(),
    }));
    graph.link_to(
        &tom_h,
        &the_polar_express,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter([
                "Hero Boy', 'Father', 'Conductor', 'Hobo', 'Scrooge', 'Santa Claus".into(),
            ]),
        }),
    );
    graph.link_to(&robert_z, &the_polar_express, RelationType::Directed);

    let aleagueof_their_own = graph.add(NodeType::Movie(Movie {
        title: "A League of Their Own".into(),
        released: 1992,
        tagline: "Once in a lifetime you get a chance to do something different.".into(),
    }));
    let madonna = graph.add(NodeType::Person(Person {
        name: "Madonna".into(),
        born: 1954,
    }));
    let geena_d = graph.add(NodeType::Person(Person {
        name: "Geena Davis".into(),
        born: 1956,
    }));
    let lori_p = graph.add(NodeType::Person(Person {
        name: "Lori Petty".into(),
        born: 1963,
    }));
    let penny_m = graph.add(NodeType::Person(Person {
        name: "Penny Marshall".into(),
        born: 1943,
    }));
    graph.link_to(
        &tom_h,
        &aleagueof_their_own,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Jimmy Dugan".into()]),
        }),
    );
    graph.link_to(
        &geena_d,
        &aleagueof_their_own,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Dottie Hinson".into()]),
        }),
    );
    graph.link_to(
        &lori_p,
        &aleagueof_their_own,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Kit Keller".into()]),
        }),
    );
    graph.link_to(
        &rosie_o,
        &aleagueof_their_own,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Doris Murphy".into()]),
        }),
    );
    graph.link_to(
        &madonna,
        &aleagueof_their_own,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["\"All the Way\" Mae Mordabito".into()]),
        }),
    );
    graph.link_to(
        &bill_pax,
        &aleagueof_their_own,
        RelationType::ActedIn(ActedIn {
            roles: Vec::from_iter(["Bob Hinson".into()]),
        }),
    );
    graph.link_to(&penny_m, &aleagueof_their_own, RelationType::Directed);

    let paul_blythe = graph.add(NodeType::Person(Person {
        name: "Paul Blythe".into(),
        born: 0,
    }));
    let angela_scope = graph.add(NodeType::Person(Person {
        name: "Angela Scope".into(),
        born: 0,
    }));
    let jessica_thompson = graph.add(NodeType::Person(Person {
        name: "Jessica Thompson".into(),
        born: 0,
    }));
    let james_thompson = graph.add(NodeType::Person(Person {
        name: "James Thompson".into(),
        born: 0,
    }));
    graph.link_to(&james_thompson, &jessica_thompson, RelationType::Follows);
    graph.link_to(&angela_scope, &jessica_thompson, RelationType::Follows);
    graph.link_to(&paul_blythe, &angela_scope, RelationType::Follows);

    graph.link_to(
        &jessica_thompson,
        &cloud_atlas,
        RelationType::Reviewed(Review {
            summary: "An amazing journey".into(),
            rating: 95,
        }),
    );
    graph.link_to(
        &jessica_thompson,
        &the_replacements,
        RelationType::Reviewed(Review {
            summary: "Silly, but fun".into(),
            rating: 65,
        }),
    );
    graph.link_to(
        &james_thompson,
        &the_replacements,
        RelationType::Reviewed(Review {
            summary: "The coolest football movie ever".into(),
            rating: 100,
        }),
    );
    graph.link_to(
        &angela_scope,
        &the_replacements,
        RelationType::Reviewed(Review {
            summary: "Pretty funny at times".into(),
            rating: 62,
        }),
    );
    graph.link_to(
        &jessica_thompson,
        &unforgiven,
        RelationType::Reviewed(Review {
            summary: "Dark, but compelling".into(),
            rating: 85,
        }),
    );
    graph.link_to(
        &jessica_thompson,
        &the_birdcage,
        RelationType::Reviewed(Review {
            summary: "Slapstick redeemed only by the Robin Williams and Gene Hackman's stellar performances".into(),
            rating: 45,
        }),
    );
    graph.link_to(
        &jessica_thompson,
        &the_da_vinci_code,
        RelationType::Reviewed(Review {
            summary: "A solid romp".into(),
            rating: 68,
        }),
    );
    graph.link_to(
        &james_thompson,
        &the_da_vinci_code,
        RelationType::Reviewed(Review {
            summary: "Fun, but a little far fetched".into(),
            rating: 65,
        }),
    );

    graph
}
