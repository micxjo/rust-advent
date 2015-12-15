#[derive(PartialEq)]
enum State {
    Flying,
    Resting,
}

pub struct Reindeer {
    speed: u32,
    stamina: u32,
    rest: u32,
    period: u32,
    state: State,
    flown: u32,
}

impl Reindeer {
    pub fn new(speed: u32, stamina: u32, rest: u32) -> Reindeer {
        Reindeer {
            speed: speed,
            stamina: stamina,
            rest: rest,
            period: 0,
            state: State::Flying,
            flown: 0,
        }
    }

    fn value(&self) -> u32 {
        self.speed * self.flown
    }

    fn tick(&mut self) {
        self.period += 1;
        if self.state == State::Flying {
            self.flown += 1;
            if self.period >= self.stamina {
                self.period = 0;
                self.state = State::Resting;
            }
        } else {
            if self.period >= self.rest {
                self.period = 0;
                self.state = State::Flying;
            }
        }
    }
}

pub fn run(reindeer: &mut Vec<Reindeer>, time: u32) -> u32 {
    for _ in 0..time {
        for r in reindeer.iter_mut() {
            r.tick();
        }
    }
    reindeer.iter().map(|r| r.value()).max().unwrap_or(0)
}

pub fn run_part2(reindeer: &mut Vec<Reindeer>, time: u32) -> u32 {
    let mut points: Vec<_> = (0..reindeer.len()).map(|_| 0).collect();
    for _ in 0..time {
        let mut tick_flown: Vec<_> = (0..reindeer.len()).map(|_| 0).collect();
        for i in 0..reindeer.len() {
            let r = reindeer.get_mut(i).unwrap();
            r.tick();
            tick_flown[i] = r.value();
        }

        let max_flown = tick_flown.iter().cloned().max().unwrap_or(0);
        for i in 0..tick_flown.len() {
            if tick_flown[i] == max_flown {
                points[i] = points[i] + 1;
            }
        }
    }

    points.into_iter().max().unwrap_or(0)
}

fn reset(reindeer: &mut Vec<Reindeer>) {
    for r in reindeer.iter_mut() {
        r.flown = 0;
        r.period = 0;
        r.state = State::Flying;
    }
}

pub fn process_file(path: &str) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let mut reindeer = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split(" ").collect();
        reindeer.push(Reindeer::new(words[3].parse::<u32>().unwrap(),
                                    words[6].parse::<u32>().unwrap(),
                                    words[13].parse::<u32>().unwrap()));
    }
    println!("Part 1: {}", run(&mut reindeer, 2503));

    reset(&mut reindeer);
    println!("Part 2: {}", run_part2(&mut reindeer, 2503));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let comet = Reindeer::new(14, 10, 127);
        let dancer = Reindeer::new(16, 11, 162);
        let mut vec = vec![comet, dancer];
        assert_eq!(run(&mut vec, 1000), 1120);
    }

    #[test]
    fn test_run_part2() {
        let comet = Reindeer::new(14, 10, 127);
        let dancer = Reindeer::new(16, 11, 162);
        let mut vec = vec![comet, dancer];
        assert_eq!(run_part2(&mut vec, 1000), 689);
    }

}
