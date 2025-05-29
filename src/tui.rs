use anyhow::Result;
use cliclack::{Confirm, Input, MultiSelect, Select};
use colored::Colorize;

use crate::gpa::{Lecture, GPA};

pub struct Tui;

// FIX: quitting the prompt causes `Error: operation interrupted` to bubble up till main
impl Tui {
    pub fn start(gpa: &mut GPA) -> Result<()> {
        ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

        let _ = cliclack::intro("GPA-RS – personal GPA tracker");

        let selected_mode = Select::new(format!(
            "Choose an action: {}",
            "(↑/↓ to move, Enter to confirm)".dimmed()
        ))
        .item("add", "Add a course", "")
        .item("edit", "Edit a course", "")
        .item("delete", "Delete a course", "")
        .item("config", "Edit GPA config", "")
        .item("exit", "Exit", "")
        .interact()?;

        match selected_mode {
            "add" => Self::add(gpa)?,
            "edit" => Self::edit(gpa)?,
            "delete" => Self::delete(gpa)?,
            "config" => Self::config(gpa)?,
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

        let credits: u8 = Input::new("Credits:").placeholder("6").interact()?;

        let semester: u8 = Input::new("Semester number:").placeholder("1").interact()?;

        let grade: Option<f32> =
            Input::new(format!("Grade: {}", "(leave blank if not graded)".dimmed()))
                .required(false)
                .interact::<String>()?
                .parse()
                .ok();

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

        let _ = cliclack::outro("Course added.");

        Ok(())
    }

    fn edit(gpa: &mut GPA) -> Result<()> {
        if gpa.lectures.is_empty() {
            let _ = cliclack::outro_cancel("No courses yet.");
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
                    // TODO: display additional metadata
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

        let fields = MultiSelect::new(format!(
            "Select the fields you want to change: {}",
            "(space to toggle)".dimmed()
        ))
        .item("title", "Title", "")
        .item("credits", "Credits", "")
        .item("semester", "Semester", "")
        .item("grade", "Grade", "")
        .item("completed", "Completed", "")
        .interact()?;

        gpa.update_lecture(idx, |lec| {
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
                    .unwrap_or_else(|_| lec.title.clone());
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
                let g: String =
                    Input::new(format!("New grade: {}", "(leave blank to clear)".dimmed()))
                        .required(false)
                        .interact()
                        .unwrap();
                lec.grade = if g.trim().is_empty() {
                    None
                } else {
                    Some(g.parse::<f32>().unwrap())
                };
            }
            if fields.contains(&"completed") {
                lec.completed = Confirm::new("Completed?")
                    .initial_value(lec.completed)
                    .interact()
                    .unwrap_or(lec.completed);
            }
        })?;

        let _ = cliclack::outro("Changes saved.");

        Ok(())
    }

    fn delete(gpa: &mut GPA) -> Result<()> {
        if gpa.lectures.is_empty() {
            let _ = cliclack::outro_cancel("Nothing to delete.");
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
            let _ = cliclack::outro("Course deleted.");
        }

        Ok(())
    }

    fn config(gpa: &mut GPA) -> Result<()> {
        let fields = MultiSelect::new(format!(
            "Select the configuration you want to change: {}",
            "(space to toggle)".dimmed()
        ))
        .item(
            "target_credits",
            "Target credits",
            format!("currently {}", gpa.target_credits),
        )
        .item(
            "target_grade",
            "Target grade",
            format!("currently {:.2}", gpa.target_grade),
        )
        .interact()?;

        if fields.contains(&"target_credits") {
            gpa.target_credits = Input::new("New target credits:")
                .placeholder(&gpa.target_credits.to_string())
                .interact()?;
        }
        if fields.contains(&"target_grade") {
            gpa.target_grade = Input::new("New target grade:")
                .placeholder(&format!("{:.2}", gpa.target_grade))
                .interact()?;
        }

        gpa.save()?;
        let _ = cliclack::outro("GPA config updated.");
        Ok(())
    }
}
