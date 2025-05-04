use labs::*;

use rules::load_rules;
use sources::html::load_school;

fn main() -> anyhow::Result<()> {
    let school = load_school("input/school.html", "input/labs.json")?;
    let rules = load_rules("input/rules.json")?;
    let solution = solver::solve(&school, &rules);
    export::html(&school, solution);
    Ok(())
}
