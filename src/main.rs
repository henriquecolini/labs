use labs::*;

use rules::load_rules;
use sources::html::load_school;

fn main() -> anyhow::Result<()> {
    let school = load_school("input/school.html")?;
    let rules = load_rules("input/rules.json")?;
    let organized = solver::solve(&rules, &school);
    export::html(&school, organized);
    Ok(())
}
