use crate::member::Member;

pub struct Group {
    members: Vec<Member>,
    round_priorities: Vec<RoundPriorities>,
    pub total_weight: u64,
}

#[derive(Clone)]
pub struct RoundPriorities { 
    priorities: Vec<u64>, 
    proposer_index: usize,
}

impl RoundPriorities {
    pub fn new(members: &Vec<Member>) -> Self {
        let mut priorities = Vec::with_capacity(members.len());
        let mut highest_weight = 0;
        let mut proposer_index = 0;
        for (index, member) in members.iter().enumerate() {
            priorities.push(member.weight);
            if member.weight > highest_weight {
                highest_weight = member.weight;
                proposer_index = index;
            }
        }
        RoundPriorities {
            priorities,
            proposer_index,
        }
    }
}


impl Group {
    pub fn new(members: Vec<Member>) -> Self {
        let roundPriorities = RoundPriorities::new(&members);
        Self::new_with_priorities(members, roundPriorities)
    }

    pub fn new_with_priorities(members: Vec<Member>, priorities: RoundPriorities) -> Self {
        let total_weight = members.iter().map(|m| m.weight).sum();
        let round_priorities = vec![priorities];
        Self {
            members,
            round_priorities,
            total_weight,
        }
    }

    pub fn member(&self, index: usize) -> Member {
        self.members[index]
    }

    pub fn proposer(mut self, round: u32) -> Member {
        if round as usize >= self.round_priorities.len() {
            for _ in 0..(round as usize - self.round_priorities.len()) {
                self.round_priorities.push(self.next_round_priorities());
            }
        }

        let proposer_index = self.round_priorities[round as usize].proposer_index;
        self.members[proposer_index] 
    }

    fn next_round_priorities(&self) -> RoundPriorities {
        let mut next_priorities: Vec<u64> = Vec::with_capacity(self.members.len());
        let last_priorities = self.round_priorities.last().unwrap(); 
        let mut highest_priority= 0;
        let mut proposer_index = 0;
        for (index, member) in self.members.iter().enumerate() {
            let new_priority = last_priorities.priorities[index] + member.weight;
            if new_priority > highest_priority {
                highest_priority = new_priority;
                proposer_index = index;
            }
            next_priorities.push(new_priority);
        }
        RoundPriorities {
            priorities: next_priorities,
            proposer_index,
        }
    }
}
