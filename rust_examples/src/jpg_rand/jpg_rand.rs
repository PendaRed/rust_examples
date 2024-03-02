
// XXXJPG Note this doesnt work, bad port from Java

pub struct JpgRand {
    last_no_ : u64,
    // mask off the upper bits that are not  needed 
    bt_msk_ : u64, 
    bt1_msk_:u64,
    bt1_shft_:u32,
    bt2_msk_:u64,
    bt2_shft_:u32,
    bit_msk_:u64,
    range_:u64,
    invert_:u64,  //xor result to change     
}

/*******************************************************************
 * the array below gives the two bits and the mask and shift etc 
 * required by each for a max number of bits..
 *******************************************************************/
static SHFT1:usize =0; /* no of bits to shift */
static MSK1:usize  =1;  /* mask to & */
static SHFT2:usize =2; /* no of bits to shift */
static MSK2:usize  =3;  /* mask to & */
static BT_MSK:usize=4;/* mask to & the finished shifted number */


static BIT_PATS: [[u32; 5]; 9] = [
  [0,0x0001,2,0x0004,0x0007],        /* no bits 3 */
  [0,0x0001,3,0x0008,0x000F],        /* no bits 4 */
  [1,0x0002,4,0x0010,0x001F],        /* no bits 5 */
  [0,0x0001,5,0x0020,0x003F],        /* no bits 6 */
  [0,0x0001,6,0x0040,0x007F],        /* no bits 7 */
  [3,0x0008,8,0x0100,0x01FF],        /* no bits 9 */
  [2,0x0004,9,0x0200,0x03FF],        /* no bits 10 */
  [1,0x0002,10,0x0400,0x07FF],       /* no bits 11 */
  [0,0x0001,14,0x4000,0x7FFF]        /* no bits 15 */
];

/************************************************************************
 * Not all numbers of bits have a polynomial non repeating sequence,
 * these must instead use the next highest that does have a sequence
 * This array maps bits 0 - 15 onto the next highest bit pattern info
 * as stored in bit_patts above
 ************************************************************************/
static BIT_TO_PAT:[i32; 17]=
[
  0,   /* 0 number of bits, use bit pattern for 3 bits */
  0,   /* bit pattern for 3 bits */
  0,   /* bit pattern for 3 bits */
  0,   /* bit pattern for 3 bits */
  1,   /* 4 bits, use bit pattern for 4 */
  2,   /* 5bits */
  3,   /* 6 bits */
  4,   /* 7 bits */
  5,   /* 9 bits */
  5,   /* 9 bits */
  6,   /* 10 bits */
  7,   /* 11 bits */
  8,   /* 15 bits */
  8,   /* 15 bits */
  8,   /* 15 bits */
  8,   /* 15 bits */
  8,   /* 15 bits */
];

impl JpgRand {
    /**
     * ctor
     *
     *   Initialise the polynomial sequence for the seed and range specified 
     * by the caller.
     * @param  i_seed   The seed value to be used
     * @param  i_range  The range of the value from 0
     */
    pub fn new(i_seed:u64, i_range:u64) -> Self {
        let mut obj = JpgRand{
            last_no_ : 0,
            // mask off the upper bits that are not  needed 
            bt_msk_ : 0, 
            bt1_msk_:0,
            bt1_shft_:0,
            bt2_msk_:0,
            bt2_shft_:0,
            bit_msk_:0,
            range_:0,
            invert_:0,  //xor result to change     
        };
        obj.reset(i_seed, i_range);
        obj
    } 

    /**
     * The sequence can be reset as required
    * @param  i_seed   The seed value to be used
    * @param  i_range  The range of the value from 0
    */
    /// #Panics
    /// If out of range error
    pub fn reset(&mut self, i_seed:u64, i_range:u64) {
        // Note increment range here as the sequence can never generate 
        // zero. have to decrement the returned value
        self.range_ = i_range+1;

        self.last_no_ = i_seed;
        self.bt_msk_ = 0;         // mask off the upper bits that are not 
        // needed 
        self.bt1_msk_ = 0;
        self.bt1_shft_ = 0;
        self.bt2_msk_ = 0;
        self.bt2_shft_ = 0;
        self.bit_msk_ = 0;
        self.invert_ = 0;        //xor result to change sequence on each call

        // Now work out the power of two required for the range 
        let mut max_no:u64 = 0x1;
        let mut no_bits:usize = 1;
        
        // max_no is just used as loop terminator 
        while max_no<self.range_ {
                no_bits+=1;
                max_no = max_no<<1;
                self.bt_msk_ = (self.bt_msk_<<1) | 1;
        }
        if no_bits> BIT_TO_PAT.len() {
            panic!("jpg_rand range is TOO BIG!! Max is 16 bits");
        }
        
        // use temporary bit mask to mask off the inersion bits
        self.invert_ &=self.bt_msk_;
        
        if no_bits<3 {
            no_bits=3;
        }

        // derive the mask and shift required from the number of bits and 
        // the initialised arrays at the start of this file. first find 
        // index into the patterns array 
        let itmp = BIT_TO_PAT[no_bits] as usize;

        self.bt1_shft_ = BIT_PATS[itmp][SHFT1] as u32;
        self.bt1_msk_ = BIT_PATS[itmp][MSK1] as u64;
        self.bt2_shft_ = BIT_PATS[itmp][SHFT2] as u32;
        self.bt2_msk_ = BIT_PATS[itmp][MSK2] as u64;
        self.bt_msk_ = BIT_PATS[itmp][BT_MSK] as u64;
    } // end of reset

    /**
     *   This function derives the next non repeating pseudo random number 
     *   in a sequence.  The number returned will be from 0 to the range
     *   specified
     * @returns The next random number in the sequence
     */
    pub fn next_rand(&mut self) -> u64 {
        let mut no:u64 = self.range_ +1;
        let mut tmp:u64;

        // loop through sequence until get the next number within the range 
        while no >= self.range_ {
            tmp = self.last_no_ & self.bt1_msk_ >>self.bt1_shft_ ^ 
                  self.last_no_ & self.bt2_msk_ >>self.bt2_shft_ ^ 0;
        

            no = self.last_no_<<1 &self.bt_msk_ | tmp;
        
            self.last_no_ = no;
            // this randomises the sequence and causes possible problems 
            // when no == invert                                         
            if no !=self.invert_ {
                no = no^self.invert_;
            }
        } // end of while not in range
        
        no-1
    }  // end of nextRand

    #[allow(dead_code)]
    pub fn next_i32(&mut self) -> i32 {
        self.next_rand() as i32
    }
    pub fn next_usize(&mut self) -> usize {
        self.next_rand() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_reating_rands() {
        let mut rnd =  JpgRand::new(10,10);
        let mut res = [0;10];
        for _i in 1..10 {
            let r = rnd.next_usize();
            res[r] += 1;
            println!("{}",r);
        }
        for i in 0..9 {
            assert_eq!(1, res[i]);
        }
//        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}