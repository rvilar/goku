pub trait Average {
    fn avg(&self) -> u64;
}

impl Average for Vec<Result> {
    fn avg(&self) -> u64 {
        let total: u64 = self.iter().map(|r| r.duration).sum();
        let size: u64 = self.iter().len() as u64;
        total / size
    }
}

#[derive(Debug)]
pub struct Result {
    pub status: u16,
    pub duration: u64,
}

#[derive(Debug)]
pub struct Report {
    results: Vec<Result>,
}

impl Report {
    pub fn new() -> Self {
        Report { results: vec![] }
    }
    pub fn total(&self) -> usize {
        self.results.len()
    }

    pub fn avg(&self) -> u64 {
        self.results.avg()
    }

    pub fn add_result(&mut self, result: Result) {
        self.results.push(result);
    }
}
