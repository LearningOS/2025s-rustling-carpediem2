// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


enum Grade{
    Numeric(f32),
    Alphabetic(String),
}
pub struct ReportCard {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {

        // 根据成绩类型生成相应的字符串表示
        let grade_str = match &self.grade {
            Grade::Numeric(grade) => format!("{}", grade),
            Grade::Alphabetic(grade) => format!("{}", grade),
        };

        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, grade_str)
    }
}
/*
定义 Grade 枚举：
    我们定义了一个 Grade 枚举，它可以是 Numeric(f32)（数值等级）或 Alphabetic(String)（字母等级）。这使得 grade 字段能够存储两种类型的成绩。
修改 ReportCard 结构体：
    将 grade 字段的类型从 f32 改为 Grade，以支持新的成绩类型。
修改 print 方法：
    在 print 方法中，我们使用 match 表达式来检查 grade 的类型。
    如果是 Numeric 类型，我们直接将其转换为字符串。
    如果是 Alphabetic 类型，我们同样将其转换为字符串。
    最后，我们使用 format! 宏生成报告卡的字符串表示。
更新测试：
在 generate_alphabetic_report_card 测试中，我们将 grade 设置为 Grade::Alphabetic("A+".to_string())，以测试字母等级的支持。
这些修改使得 ReportCard 结构体能够支持两种类型的成绩，并且 print 方法能够正确地生成包含字母等级的报告卡字符串。
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numeric(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            //grade: 2.1,
            grade:Grade::Alphabetic("A+".to_string()) ,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
