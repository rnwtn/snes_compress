pub fn pc_to_lorom(pc: u32) -> Result<u32, &'static str> {
    if pc >= 0x400000 {
        Err("pc out of bounds")
    } else {
        let lower_word = pc & 0x00FFFF | 0x8000;
        let lorom = (((pc / 0x8000) << 16) + 0x800000) | lower_word;
        Ok(lorom)
    }
}

pub fn lorom_to_pc(lorom: u32) -> Result<u32, &'static str> {
    if !(0x800000..=0xFFFFFF).contains(&lorom) {
        Err("lorom out of bounds")
    } else {
        Ok((lorom & 0x7F0000) >> 1 | (lorom & 0x7FFF))
    }
}

#[cfg(test)]
mod adress_converter_tests {
    use super::*;

    #[test]
    fn pc_to_lorom_works() {
        let result = pc_to_lorom(0x2156E8);
        assert_eq!(result, Ok(0xC2D6E8));
    }

    #[test]
    fn pc_to_lorom_lowerbound() {
        let result = pc_to_lorom(0x000000);
        assert_eq!(result, Ok(0x808000));
    }

    #[test]
    fn pc_to_lorom_upperbound() {
        let result = pc_to_lorom(0x3FFFFF);
        assert_eq!(result, Ok(0xFFFFFF));
    }

    #[test]
    fn pc_to_lorom_out_of_bounds() {
        let result = pc_to_lorom(0x400000);
        assert_eq!(result, Err("pc out of bounds"));
    }

    #[test]
    fn lorom_to_pc_works() {
        let result = lorom_to_pc(0xC2D6E8);
        assert_eq!(result, Ok(0x2156E8));
    }

    #[test]
    fn lorom_to_pc_lowerbound() {
        let result = lorom_to_pc(0x808000);
        assert_eq!(result, Ok(0x000000));
    }

    #[test]
    fn lorom_to_pc_upperbound() {
        let result = lorom_to_pc(0xFFFFFF);
        assert_eq!(result, Ok(0x3FFFFF));
    }

    #[test]
    fn lorom_to_pc_out_of_bounds_lower() {
        let result = lorom_to_pc(0x7FFFFF);
        assert_eq!(result, Err("lorom out of bounds"));
    }

    #[test]
    fn lorom_to_pc_out_of_bounds_upper() {
        let result = lorom_to_pc(0x1000000);
        assert_eq!(result, Err("lorom out of bounds"));
    }
}
