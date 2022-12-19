use std::collections::HashMap;

use crate::memory::WordSize;

struct Char {
    glyph: char,
    bitmap: Vec<u8>
}

impl Char {
    fn new(glyph: char, bitmap: Vec<u8>) -> Char {
        Char {
            glyph,
            bitmap,
        }
    }
}

pub struct CharMap {
    cmap: HashMap<WordSize, Char>
}

impl CharMap {
    pub fn new() -> CharMap {
        let mut cmap: HashMap<WordSize, Char> = HashMap::new();
        cmap.insert(0, Char::new('█', vec![63,63,63,63,63,63,63,63,63,0,0]));
        cmap.insert(32,Char::new(' ', vec![0,0,0,0,0,0,0,0,0,0,0]));
        cmap.insert(33,Char::new('!', vec![12,30,30,30,12,12,0,12,12,0,0]));
        cmap.insert(34,Char::new('"', vec![54,54,20,0,0,0,0,0,0,0,0]));
        cmap.insert(35,Char::new('#', vec![0,18,18,63,18,18,63,18,18,0,0]));
        cmap.insert(36,Char::new('$', vec![12,30,51,3,30,48,51,30,12,12,0]));
        cmap.insert(37,Char::new('%', vec![0,0,35,51,24,12,6,51,49,0,0]));
        cmap.insert(38,Char::new('&', vec![12,30,30,12,54,27,27,27,54,0,0]));
        cmap.insert(39,Char::new('\'', vec![12,12,6,0,0,0,0,0,0,0,0]));
        cmap.insert(40,Char::new('(', vec![24,12,6,6,6,6,6,12,24,0,0]));
        cmap.insert(41,Char::new(')', vec![6,12,24,24,24,24,24,12,6,0,0]));
        cmap.insert(42,Char::new('*', vec![0,0,0,51,30,63,30,51,0,0,0]));
        cmap.insert(43,Char::new('+', vec![0,0,0,12,12,63,12,12,0,0,0]));
        cmap.insert(44,Char::new(',', vec![0,0,0,0,0,0,0,12,12,6,0]));
        cmap.insert(45,Char::new('-', vec![0,0,0,0,0,63,0,0,0,0,0]));
        cmap.insert(46,Char::new('.', vec![0,0,0,0,0,0,0,12,12,0,0]));
        cmap.insert(47,Char::new('/', vec![0,0,32,48,24,12,6,3,1,0,0]));
        cmap.insert(48, Char::new('0', vec![12,30,51,51,51,51,51,30,12,0,0])); 
        cmap.insert(49, Char::new('1', vec![12,14,15,12,12,12,12,12,63,0,0])); 
        cmap.insert(50, Char::new('2', vec![30,51,48,24,12,6,3,51,63,0,0]));   
        cmap.insert(51, Char::new('3', vec![30,51,48,48,28,48,48,51,30,0,0])); 
        cmap.insert(52, Char::new('4', vec![16,24,28,26,25,63,24,24,60,0,0])); 
        cmap.insert(53, Char::new('5', vec![63,3,3,31,48,48,48,51,30,0,0]));   
        cmap.insert(54, Char::new('6', vec![28,6,3,3,31,51,51,51,30,0,0]));    
        cmap.insert(55, Char::new('7', vec![63,49,48,48,24,12,12,12,12,0,0])); 
        cmap.insert(56, Char::new('8', vec![30,51,51,51,30,51,51,51,30,0,0])); 
        cmap.insert(57, Char::new('9', vec![30,51,51,51,62,48,48,24,14,0,0])); 
        cmap.insert(58, Char::new(':', vec![0,0,12,12,0,0,12,12,0,0,0]));      
        cmap.insert(59, Char::new(';', vec![0,0,12,12,0,0,12,12,6,0,0]));      
        cmap.insert(48, Char::new('0', vec![12,30,51,51,51,51,51,30,12,0,0])); 
        cmap.insert(60, Char::new('<', vec![0,0,24,12,6,3,6,12,24,0,0]));      
        cmap.insert(61, Char::new('=', vec![0,0,0,63,0,0,63,0,0,0,0]));        
        cmap.insert(62, Char::new('>', vec![0,0,3,6,12,24,12,6,3,0,0]));       
        cmap.insert(64, Char::new('@', vec![30,51,51,59,59,59,27,3,30,0,0]));  
        cmap.insert(63, Char::new('?', vec![30,51,51,24,12,12,0,12,12,0,0]));  
        cmap.insert(65, Char::new('A', vec![0,0,0,0,0,0,0,0,0,0,0]));         
        cmap.insert(66, Char::new('B', vec![31,51,51,51,31,51,51,51,31,0,0])); 
        cmap.insert(67, Char::new('C', vec![28,54,35,3,3,3,35,54,28,0,0]));    
        cmap.insert(68, Char::new('D', vec![15,27,51,51,51,51,51,27,15,0,0])); 
        cmap.insert(69, Char::new('E', vec![63,51,35,11,15,11,35,51,63,0,0])); 
        cmap.insert(70, Char::new('F', vec![63,51,35,11,15,11,3,3,3,0,0]));    
        cmap.insert(71, Char::new('G', vec![28,54,35,3,59,51,51,54,44,0,0]));  
        cmap.insert(72, Char::new('H', vec![51,51,51,51,63,51,51,51,51,0,0])); 
        cmap.insert(73, Char::new('I', vec![30,12,12,12,12,12,12,12,30,0,0])); 
        cmap.insert(74, Char::new('J', vec![60,24,24,24,24,24,27,27,14,0,0])); 
        cmap.insert(75, Char::new('K', vec![51,51,51,27,15,27,51,51,51,0,0])); 
        cmap.insert(76, Char::new('L', vec![3,3,3,3,3,3,35,51,63,0,0]));       
        cmap.insert(77, Char::new('M', vec![33,51,63,63,51,51,51,51,51,0,0])); 
        cmap.insert(78, Char::new('N', vec![51,51,55,55,63,59,59,51,51,0,0])); 
        cmap.insert(79, Char::new('O', vec![30,51,51,51,51,51,51,51,30,0,0])); 
        cmap.insert(80, Char::new('P', vec![31,51,51,51,31,3,3,3,3,0,0]));     
        cmap.insert(81, Char::new('Q', vec![30,51,51,51,51,51,63,59,30,48,0]));
        cmap.insert(82, Char::new('R', vec![31,51,51,51,31,27,51,51,51,0,0])); 
        cmap.insert(83, Char::new('S', vec![30,51,51,6,28,48,51,51,30,0,0]));  
        cmap.insert(84, Char::new('T', vec![63,63,45,12,12,12,12,12,30,0,0])); 
        cmap.insert(85, Char::new('U', vec![51,51,51,51,51,51,51,51,30,0,0])); 
        cmap.insert(86, Char::new('V', vec![51,51,51,51,51,30,30,12,12,0,0])); 
        cmap.insert(87, Char::new('W', vec![51,51,51,51,51,63,63,63,18,0,0])); 
        cmap.insert(88, Char::new('X', vec![51,51,30,30,12,30,30,51,51,0,0])); 
        cmap.insert(89, Char::new('Y', vec![51,51,51,51,30,12,12,12,30,0,0])); 
        cmap.insert(90, Char::new('Z', vec![63,51,49,24,12,6,35,51,63,0,0]));  
        cmap.insert(91, Char::new('[', vec![30,6,6,6,6,6,6,6,30,0,0]));        
        cmap.insert(92, Char::new('\\', vec![0,0,1,3,6,12,24,48,32,0,0]));      
        cmap.insert(93, Char::new(']', vec![30,24,24,24,24,24,24,24,30,0,0])); 
        cmap.insert(94, Char::new('^', vec![8,28,54,0,0,0,0,0,0,0,0]));        
        cmap.insert(95, Char::new('_', vec![0,0,0,0,0,0,0,0,0,63,0]));         
        cmap.insert(96, Char::new('`', vec![6,12,24,0,0,0,0,0,0,0,0]));        
        cmap.insert(97, Char::new('a', vec![0,0,0,14,24,30,27,27,54,0,0]));    
        cmap.insert(98, Char::new('b', vec![3,3,3,15,27,51,51,51,30,0,0]));    
        cmap.insert(99, Char::new('c', vec![0,0,0,30,51,3,3,51,30,0,0]));      
        cmap.insert(100, Char::new('d', vec![48,48,48,60,54,51,51,51,30,0,0]));
        cmap.insert(101, Char::new('e', vec![0,0,0,30,51,63,3,51,30,0,0]));    
        cmap.insert(102, Char::new('f', vec![28,54,38,6,15,6,6,6,15,0,0]));    
        cmap.insert(103, Char::new('g', vec![0,0,30,51,51,51,62,48,51,30,0])); 
        cmap.insert(104, Char::new('h', vec![3,3,3,27,55,51,51,51,51,0,0]));   
        cmap.insert(105, Char::new('i', vec![12,12,0,14,12,12,12,12,30,0,0])); 
        cmap.insert(106, Char::new('j', vec![48,48,0,56,48,48,48,48,51,30,0]));
        cmap.insert(107, Char::new('k', vec![3,3,3,51,27,15,15,27,51,0,0]));   
        cmap.insert(108, Char::new('l', vec![14,12,12,12,12,12,12,12,30,0,0]));
        cmap.insert(109, Char::new('m', vec![0,0,0,29,63,43,43,43,43,0,0]));   
        cmap.insert(110, Char::new('n', vec![0,0,0,29,51,51,51,51,51,0,0]));   
        cmap.insert(111, Char::new('o', vec![0,0,0,30,51,51,51,51,30,0,0]));   
        cmap.insert(112, Char::new('p', vec![0,0,0,30,51,51,51,31,3,3,0]));    
        cmap.insert(113, Char::new('q', vec![0,0,0,30,51,51,51,62,48,48,0]));  
        cmap.insert(114, Char::new('r', vec![0,0,0,29,55,51,3,3,7,0,0]));      
        cmap.insert(115, Char::new('s', vec![0,0,0,30,51,6,24,51,30,0,0]));    
        cmap.insert(116, Char::new('t', vec![4,6,6,15,6,6,6,54,28,0,0]));      
        cmap.insert(117, Char::new('u', vec![0,0,0,27,27,27,27,27,54,0,0]));   
        cmap.insert(118, Char::new('v', vec![0,0,0,51,51,51,51,30,12,0,0]));   
        cmap.insert(119, Char::new('w', vec![0,0,0,51,51,51,63,63,18,0,0]));   
        cmap.insert(120, Char::new('x', vec![0,0,0,51,30,12,12,30,51,0,0]));   
        cmap.insert(121, Char::new('y', vec![0,0,0,51,51,51,62,48,24,15,0]));  
        cmap.insert(122, Char::new('z', vec![0,0,0,63,27,12,6,51,63,0,0]));    
        cmap.insert(123, Char::new('{', vec![56,12,12,12,7,12,12,12,56,0,0])); 
        cmap.insert(124, Char::new('|', vec![12,12,12,12,12,12,12,12,12,0,0]));
        cmap.insert(125, Char::new('}', vec![7,12,12,12,56,12,12,12,7,0,0]));  
        cmap.insert(126, Char::new('~', vec![38,45,25,0,0,0,0,0,0,0,0]));      
        CharMap { cmap }
    }

    /**
     * Returns the bitmap for a valid character. 
     * Returns bitmap for black square for any invalid characters.
     */
    pub fn get_bitmap(&self, character: &WordSize) -> &Vec<u8> {
        if self.cmap.contains_key(character) {
            self.cmap.get(character).unwrap().bitmap.as_ref()
        } else {
            self.cmap.get(&(0 as i16)).unwrap().bitmap.as_ref()
        }
    }
}
