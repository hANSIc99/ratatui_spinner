pub struct Spinner {
    m_spinner   : Vec<char>,
    m_iter      : usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            m_spinner : vec!['|', '/', '-', '\\'],
            m_iter : 0,
        }
    }

}

impl Iterator for Spinner {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.m_iter += 1;
        self.m_iter &= 0x3;

        Some(self.m_spinner[self.m_iter])
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}