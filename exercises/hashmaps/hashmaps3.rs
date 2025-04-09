// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
                // Update team 1's stats
                let entry1 = scores.entry(team_1_name.clone()).or_insert(Team {
                    goals_scored: 0,
                    goals_conceded: 0,
                });
                entry1.goals_scored += team_1_score;
                entry1.goals_conceded += team_2_score;
         
                // Update team 2's stats
                let entry2 = scores.entry(team_2_name.clone()).or_insert(Team {
                    goals_scored: 0,
                    goals_conceded: 0,
                });
                entry2.goals_scored += team_2_score;
                entry2.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}


//scores.entry(team_1_name.clone())：尝试获取 scores 中以 team_1_name 为键的条目。
//.or_insert(Team { goals_scored: 0, goals_conceded: 0 })：如果 team_1_name 不存在于 scores 中，则插入一个新的 Team 实例，初始进球数和失球数均为 0。
//team_1_name.clone() 和 team_2_name.clone() 是为了将字符串的拥有权从调用者转移到 HashMap 中。如果 team_1_name 和 team_2_name 是 String 类型，这是必要的；如果是 &str 类型，则不需要克隆。
//如果 scores 是一个共享资源（例如在多线程环境中使用），需要确保对它的访问是线程安全的，可以使用 Mutex 或 RwLock 等同步原语。