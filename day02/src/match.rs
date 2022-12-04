use std::str::FromStr;

pub mod day2;

pub struct Match {
	opponent_choice: String,
	choice: String
}

impl Match {
	pub fn new(opponent: String, choice: String) -> Self {
		Self { opponent_choice: opponent, choice: choice }
	}

	pub fn selectFromOutcome(opponent: String, outcome: String) -> Self {
		let mut selection = "";
		//Rock
		if opponent == "A" {
			if outcome == "X" { //Lose 
				selection = "Z";
			}
			if outcome == "Y" { //Draw
				selection = "X";
			}
			if outcome == "Z" { //Win
				selection = "Y";
			}
		}
		//Paper
		if opponent == "B" {
			if outcome == "X" { //Lose 
				selection = "X";
			}
			if outcome == "Y" { //Draw
				selection = "Y";
			}
			if outcome == "Z" { //Win
				selection = "Z";
			}
		}
		//Scissors
		if opponent == "C" {
			if outcome == "X" { //Lose 
				selection = "Y";
			}
			if outcome == "Y" { //Draw
				selection = "Z";
			}
			if outcome == "Z" { //Win
				selection = "X";
			}
		}
		let choice = String::from_str(selection).unwrap();
		Self { opponent_choice: opponent, choice: choice }
	}

	pub fn score(&self) -> i64 {
		//Rock
		if &self.opponent_choice == "A" {
			if &self.choice == "X" {  //Rock [1] - Draw [3]
				return 4;
			}
			if &self.choice == "Y" { //Paper [2] - Win [6]
				return 8;
			}
			if &self.choice == "Z" { //Scissors [3] - Lose [0]
				return 3;
			}
		}
		//Paper
		if &self.opponent_choice == "B" {
			if &self.choice == "X" {  //Rock [1] - Lose [0]
				return 1;
			}
			if &self.choice == "Y" { //Paper [2] - Draw [3]
				return 5;
			}
			if &self.choice == "Z" { //Scissors [3] - Win [6]
				return 9;
			}
		}
		//Scissors
		if &self.opponent_choice == "C" {
			if &self.choice == "X" {  //Rock [1] - Win [6]
				return 7;
			}
			if &self.choice == "Y" { //Paper [2] - Lose [0]
				return 2;
			}
			if &self.choice == "Z" { //Scissors [3] - Draw [3]
				return 6;
			}
		}

		return 0;
	}
}