use crate::ecc::viterbi::common::*;
use crate::ecc::viterbi::encode::EncoderState;

pub struct BitDecoderState {
    trellis: Vec<[Link; 4]>,
}

impl BitDecoderState {
    /// entering the right capacity will prevent any additional memory allocations while pushing bits
    /// into the decoder
    pub fn new(capacity: usize) -> Self {
        assert!(capacity <= 127);

        Self {
            trellis: Vec::with_capacity(capacity),
        }
    }

    /// push a pair of bits to be decoded
    ///
    /// takes u8s instead of bools for conveince (just do a `bitwise and` between the mask and the byte)
    pub fn push(&mut self, s0: u8, s1: u8) {
        // if self.len() >= 127 {
        // 	panic!("TOO MANY BITS BEING DECODED BEFORE RESET")
        // }

        let bit_pair = combine(s0, s1);

        self.add_column();

        for state in self.states() {
            for (link, pos) in Link::next(state, bit_pair, self.prev_cost(state)) {
                self.add_link(link, pos);
            }
        }
    }

    fn add_column(&mut self) {
        self.trellis.push([Link::NONE; 4]);
    }

    fn states(&self) -> Vec<u8> {
        match self.len() {
            //0 => unreachable!(), // should only be called after adding first column to vec
            1 => vec![0],
            2 => vec![0, 1],
            _ => vec![0, 1, 2, 3]
        }
    }

    /// ouputs a vector of u8s where only the correct bits are set to 1
    pub fn read(&mut self, bit: u8) -> Vec<u8> {
        // TODO: figure out what to do if the decoder hasn't been given enough data
        // for now, just assert that it has
        assert!(self.len() > 1);

        let mut ans = Vec::with_capacity(self.len());

        // find the link to start from
        let mut pos = self.find_start_pos();

        // follow the links to the start and record what bit we think was encoded
        while !self.trellis.is_empty() {
            // record the bit
            ans.push(state_to_bit(pos, bit));

            // get position of next link
            pos = self.get_last_link(pos).prev_state;

            // ditch the current column, thus moving onto next column
            self.trellis.pop();
        }

        ans.reverse(); // TODO: fill array backwards instead of reversing

        ans
    }

    fn len(&self) -> usize {
        self.trellis.len()
    }

    fn get_last_trellis(&self) -> &[Link; 4] {
        unsafe { self.trellis.last().unwrap_unchecked() }
    }

    fn get_link(&self, index: usize, pos: u8) -> Link {
        self.trellis[index][pos as usize].clone()
    }

    fn get_last_link(&self, pos: u8) -> Link {
        self.get_last_trellis()[pos as usize].clone()
    }

    fn add_link(&mut self, new_link: Link, pos: u8) {
        unsafe { self.trellis.last_mut().unwrap_unchecked()[pos as usize].minimize_cost(new_link) }
    }

    fn find_start_pos(&self) -> u8 {
        unsafe { self.get_last_trellis().iter().enumerate().min_by_key(|(_, link)| link.cost).unwrap_unchecked().0 as u8 }
    }

    fn prev_cost(&self, pos: u8) -> u8 {
        match self.len() {
            //0 => unreachable!(), // should only be called after adding first column to vec
            1 => 0, // there is no previous link
            _ => {
                self.get_link(self.len() - 2, pos).cost
            }
        }
    }
}

#[derive(Clone)]
pub struct Link {
    pub prev_state: u8,

    /// enough for the decoder to consume 254 bits (yielding 127 bits) no matter what
    pub cost: u8,
}

impl Link {
    pub const NONE: Self = Link {
        prev_state: 255,
        cost: 255,
    };

    /// return the next 2 links and where the link should be placed
    pub fn next(state: u8, bit_pair: u8, prev_cost: u8) -> [(Self, u8); 2] {
        [
            Self::generate(state, bit_pair, prev_cost, 0),
            Self::generate(state, bit_pair, prev_cost, 1)
        ]
    }

    pub fn minimize_cost(&mut self, other: Self) {
        // TODO: figure out what should be done if the costs are the same...
        if self.cost > other.cost {
            *self = other;
        }
    }

    pub fn generate(state: u8, bit_pair: u8, prev_cost: u8, bit: u8) -> (Self, u8) {
        /* NOTES to self
        * the prev_state for each link is simply the state parameter
        * hamming dist is between bit_pair and what comes out of the encoder.input_byte_out function
        * the correct placement for each link is the internal state of its encoder after inputting the 1 or 0
         */

        let mut encoder: EncoderState<u8> = state.into();
        let hypothetical_bit_pair = encoder.push_return_bitpair(stretch(bit));

        (
            Self {
                prev_state: state,
                cost: prev_cost + Self::hamming_dist(bit_pair, hypothetical_bit_pair),
            },
            encoder.into()
        )
    }

    pub fn hamming_dist(a: u8, b: u8) -> u8 {
        (a ^ b).count_ones() as u8
    }
}
