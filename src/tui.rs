use anyhow::Result;
use cliclack::{Confirm, Input, MultiSelect, Select};
use colored::Colorize;

use crate::{
    file_util,
    gpa::{Lecture, GPA},
};

pub fn start(gpa: &mut GPA) -> Result<()> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    cliclack::intro("GPA-RS – personal GPA tracker")?;

    let selected_mode = Select::new(format!(
        "Choose an action: {}",
        "(↑/↓ to move, Enter to confirm, Esc to cancel)".dimmed()
    ))
    .item("add", "Add a course", "")
    .item("edit", "Edit a course", "")
    .item("delete", "Delete a course", "")
    .item("config", "Edit GPA config", "")
    .item("exit", "Exit", "")
    .interact()?;

    match selected_mode {
        "add" => add(gpa)?,
        "edit" => edit(gpa)?,
        "delete" => delete(gpa)?,
        "config" => config(gpa)?,
        "exit" => {}
        _ => unreachable!(),
    }

    Ok(())
}

fn add(gpa: &mut GPA) -> Result<()> {
    let mut title: String = Input::new("Course Title:")
        .placeholder("Algorithms & Data Structures")
        .validate(|input: &String| {
            if input.trim().is_empty() {
                Err("Title is required.")
            } else {
                Ok(())
            }
        })
        .interact()?;
    title = title.trim().to_string();

    let credits: u16 = Input::new("Credits:").placeholder("6").interact()?;

    let semester: u16 = Input::new("Semester number:").placeholder("1").interact()?;

    let grade_str: String =
        Input::new(format!("Grade: {}", "(leave blank if not graded)".dimmed()))
            .required(false)
            .validate(|s: &String| {
                if s.trim().is_empty() {
                    return Ok(());
                }
                parse_non_negative_grade(s)
                    .map(|_| ())
                    .map_err(|e| e.to_string())
            })
            .interact()?;
    let grade = if grade_str.trim().is_empty() {
        None
    } else {
        Some(parse_non_negative_grade(&grade_str)?)
    };

    let completed: bool = Confirm::new("Mark course as completed?")
        .initial_value(grade.is_some())
        .interact()?;

    gpa.add_lecture(Lecture {
        title,
        credits,
        semester,
        grade,
        completed,
    })?;
    if let Err(e) = gpa.save(file_util::get_config_path()) {
        cliclack::outro_cancel("Could not save course.")?;
        return Err(e);
    }
    cliclack::outro("Course added.")?;
    Ok(())
}

fn edit(gpa: &mut GPA) -> Result<()> {
    if gpa.lectures.is_empty() {
        cliclack::outro_cancel("No courses yet.")?;
        return Ok(());
    }

    let options = gpa
        .lectures
        .iter()
        .enumerate()
        .map(|(i, l)| {
            (
                i.to_string(),
                l.title.clone(),
                format!("S{} - {} Credits", l.semester, l.credits),
            )
        })
        .collect::<Vec<_>>();

    let idx: usize = Select::new(format!(
        "Pick a course to edit: {}",
        "(type to filter)".dimmed()
    ))
    .items(&options)
    .max_rows(10)
    .filter_mode()
    .interact()?
    .parse()?;

    let lec = gpa.get_lecture_mut(idx)?;

    let fields = MultiSelect::new(format!(
        "Select the fields you want to change: {}",
        "(space to toggle)".dimmed()
    ))
    .item("title", "Title", format!("current {}", lec.title))
    .item("credits", "Credits", format!("current {}", lec.credits))
    .item("semester", "Semester", format!("current {}", lec.semester))
    .item(
        "grade",
        "Grade",
        format!(
            "current {}",
            lec.grade
                .map_or("not graded".to_string(), |g| g.to_string())
        ),
    )
    .item(
        "completed",
        "Completed",
        format!("current {}", if lec.completed { "yes" } else { "no" }),
    )
    .interact()?;

    if fields.contains(&"title") {
        let title = Input::new("New title:")
            .placeholder(&lec.title)
            .validate(|input: &String| {
                if input.trim().is_empty() {
                    Err("Title is required.")
                } else {
                    Ok(())
                }
            })
            .interact()
            .unwrap_or(lec.title.clone());
        lec.title = title.trim().to_string();
    }
    if fields.contains(&"credits") {
        lec.credits = Input::new("New credits:")
            .placeholder(&lec.credits.to_string())
            .interact()
            .unwrap_or(lec.credits);
    }
    if fields.contains(&"semester") {
        lec.semester = Input::new("New semester:")
            .placeholder(&lec.semester.to_string())
            .interact()
            .unwrap_or(lec.semester);
    }
    if fields.contains(&"grade") {
        let grade_str: String =
            Input::new(format!("New grade: {}", "(leave blank to clear)".dimmed()))
                .required(false)
                .placeholder(&if let Some(g) = lec.grade {
                    format!("{:.2}", g)
                } else {
                    "".to_string()
                })
                .validate(|s: &String| {
                    if s.trim().is_empty() {
                        return Ok(());
                    }
                    parse_non_negative_grade(s)
                        .map(|_| ())
                        .map_err(|e| e.to_string())
                })
                .interact()?;
        lec.grade = if grade_str.trim().is_empty() {
            None
        } else {
            Some(parse_non_negative_grade(&grade_str)?)
        };
    }
    if fields.contains(&"completed") {
        lec.completed = Confirm::new("Completed?")
            .initial_value(lec.completed)
            .interact()
            .unwrap_or(lec.completed);
    }

    if let Err(e) = gpa.save(file_util::get_config_path()) {
        cliclack::outro_cancel("Could not save changes.")?;
        return Err(e);
    }
    cliclack::outro("Changes saved.")?;
    Ok(())
}

fn delete(gpa: &mut GPA) -> Result<()> {
    if gpa.lectures.is_empty() {
        cliclack::outro_cancel("Nothing to delete.")?;
        return Ok(());
    }

    let options = gpa
        .lectures
        .iter()
        .enumerate()
        .map(|(idx, lec)| (idx.to_string(), lec.title.clone(), ""))
        .collect::<Vec<_>>();

    let idx: usize = Select::new("Select a course to delete:")
        .items(&options)
        .max_rows(10)
        .filter_mode()
        .interact()?
        .parse()?;

    let lec = gpa
        .lectures
        .get(idx)
        .ok_or_else(|| anyhow::anyhow!("No lecture at index {idx}"))?;

    if Confirm::new(format!("Delete \"{}\"?  This cannot be undone.", lec.title)).interact()? {
        gpa.delete_lecture(idx)?;
        if let Err(e) = gpa.save(file_util::get_config_path()) {
            cliclack::outro_cancel("Could not delete course.")?;
            return Err(e);
        }
        cliclack::outro("Course deleted.")?;
    }

    Ok(())
}

fn config(gpa: &mut GPA) -> Result<()> {
    let fields = MultiSelect::new(format!(
        "Select the configurations you want to update: {}",
        "(space to toggle)".dimmed()
    ))
    .item(
        "target_average",
        "Target average",
        format!("current {:.2}", gpa.target_average),
    )
    .item(
        "credit_goal",
        "Credit goal",
        format!("current {}", gpa.grading_system.credit_goal),
    )
    .item(
        "ignore_failed",
        "Ignore failed courses in average",
        if gpa.grading_system.ignore_failed {
            "current enabled"
        } else {
            "current disabled"
        },
    )
    .item(
        "lower_is_better",
        "Lower numbers mean better grades",
        if gpa.grading_system.lower_is_better {
            "current enabled"
        } else {
            "current disabled"
        },
    )
    .item(
        "pass_mark",
        "Numeric pass mark",
        format!("current {:.2}", gpa.grading_system.pass_mark),
    )
    .interact()?;

    if fields.contains(&"target_average") {
        gpa.target_average = Input::new("New target average:")
            .placeholder(&format!("{:.2}", gpa.target_average))
            .validate(|s: &String| {
                if s.trim().is_empty() {
                    return Ok(());
                }
                parse_non_negative_grade(s)
                    .map(|_| ())
                    .map_err(|e| e.to_string())
            })
            .interact()?;
    }
    if fields.contains(&"credit_goal") {
        gpa.grading_system.credit_goal = Input::new("New credit goal:")
            .placeholder(&gpa.grading_system.credit_goal.to_string())
            .interact()?;
    }
    if fields.contains(&"ignore_failed") {
        gpa.grading_system.ignore_failed = Confirm::new("Ignore failed courses in average?")
            .initial_value(gpa.grading_system.ignore_failed)
            .interact()?;
    }
    if fields.contains(&"lower_is_better") {
        gpa.grading_system.lower_is_better =
            Confirm::new("Do lower numbers represent better grades?")
                .initial_value(gpa.grading_system.lower_is_better)
                .interact()?;
    }
    if fields.contains(&"pass_mark") {
        gpa.grading_system.pass_mark = Input::new("New numeric pass mark:")
            .placeholder(&format!("{:.2}", gpa.grading_system.pass_mark))
            .validate(|s: &String| {
                if s.trim().is_empty() {
                    return Ok(());
                }
                parse_non_negative_grade(s)
                    .map(|_| ())
                    .map_err(|e| e.to_string())
            })
            .interact()?;
    }

    if let Err(e) = gpa.save(file_util::get_config_path()) {
        cliclack::outro_cancel("Could not save configuration.")?;
        return Err(e);
    }
    cliclack::outro("Configuration saved.")?;
    Ok(())
}

fn parse_non_negative_grade(input: &str) -> Result<f32> {
    let g: f32 = input
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Value must be a decimal number"))?;
    if !g.is_finite() {
        anyhow::bail!("Value can’t be infinite nor NaN");
    } else if g.signum() == -1.0 {
        anyhow::bail!("Value can’t be negative");
    } else {
        Ok(g)
    }
}
