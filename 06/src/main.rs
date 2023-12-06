// #![allow(unused_must_use, dead_code, unused_mut, unused_variables, unused_imports)]

#[derive(Debug)]
struct Race {
    time: u64,
    distance_record: u64,
    ways_to_win: u64,
}
impl Race {
    fn new(time: u64, distance_record: u64) -> Self {
        let mut ways_to_win: u64 = 0;
        for i in 0..=time { if i*(time-i) > distance_record { ways_to_win += 1 }; }
        Self { time, distance_record, ways_to_win }
    }
    fn number_of_ways_to_win(&self) -> u64 {
        for i in 0..=(self.time/2) {
            if i*(self.time-i) > self.distance_record {
                return 1 + self.time - (i*2);
            }
        }
        0
    }
}

fn main() {

    let races = [
        Race::new(40, 219), 
        Race::new(81, 1012), 
        Race::new(77, 1365), 
        Race::new(72, 1089)
    ];

    let mut product: u64 = 1;
    for r in races.iter() {
        product *= r.ways_to_win;
    }
    println!("Multiplying the total number of ways you could win these races yields: {}", product);

    let big_race = Race::new(40817772, 219101213651089);
    println!("The number of ways you beat the record in this one much longer race is {}", big_race.number_of_ways_to_win());
}
