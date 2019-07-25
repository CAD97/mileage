var N=null,E="",T="t",U="u",searchIndex={};
var R=["charrange","result","try_from","try_into","borrow","borrow_mut","type_id","typeid","to_owned","clone_into","into_iter","into_par_iter","ordering","formatter","CharRange"];

searchIndex["char_range"]={"doc":"Character Range","i":[[0,"range","char_range",E,N,N],[3,"Iter","char_range::range","An iterator over a range of unicode code points.",N,N],[3,"ParIter",E,"A parallel iterator over a range of unicode code points.",N,N],[3,R[14],E,"An inclusive range of characters.",N,N],[12,"low",E,"The lowest character in this range (inclusive).",0,N],[12,"high",E,"The highest character in this range (inclusive).",0,N],[11,"closed",E,"A closed range `low..=high`.",0,[[["char"]],[R[0]]]],[11,"empty",E,"A canonical empty range.",0,[[],[R[0]]]],[11,"contains",E,"Does this range include a character?",0,[[["char"]],["bool"]]],[11,"cmp_char",E,"Determine the ordering of a character compared to this…",0,[[["char"]],[R[12]]]],[11,"len",E,"How many characters are in this range?",0,[[],["usize"]]],[11,"is_empty",E,"Is this range empty?",0,[[],["bool"]]],[11,"iter",E,"An iterator over this range.",0,[[],["iter"]]],[11,R[2],E,E,1,[[[U]],[R[1]]]],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[3],E,E,1,[[],[R[1]]]],[11,R[10],E,E,1,[[],["i"]]],[11,R[4],E,E,1,[[["self"]],[T]]],[11,R[5],E,E,1,[[["self"]],[T]]],[11,R[6],E,E,1,[[["self"]],[R[7]]]],[11,"par_bridge",E,E,1,[[],["iterbridge"]]],[11,R[8],E,E,1,[[["self"]],[T]]],[11,R[9],E,E,1,[[[T],["self"]]]],[11,R[2],E,E,2,[[[U]],[R[1]]]],[11,"into",E,E,2,[[],[U]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[3],E,E,2,[[],[R[1]]]],[11,R[4],E,E,2,[[["self"]],[T]]],[11,R[5],E,E,2,[[["self"]],[T]]],[11,R[6],E,E,2,[[["self"]],[R[7]]]],[11,R[11],E,E,2,[[],[T]]],[11,R[8],E,E,2,[[["self"]],[T]]],[11,R[9],E,E,2,[[[T],["self"]]]],[11,R[2],E,E,0,[[[U]],[R[1]]]],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[3],E,E,0,[[],[R[1]]]],[11,R[10],E,E,0,[[],["i"]]],[11,R[4],E,E,0,[[["self"]],[T]]],[11,R[5],E,E,0,[[["self"]],[T]]],[11,R[6],E,E,0,[[["self"]],[R[7]]]],[11,R[11],E,E,0,[[],[T]]],[11,"par_iter",E,E,0,[[["self"]]]],[11,R[8],E,E,0,[[["self"]],[T]]],[11,R[9],E,E,0,[[[T],["self"]]]],[11,"eq",E,E,0,[[["self"]],["bool"]]],[11,"partial_cmp",E,E,0,[[["self"]],[["option",[R[12]]],[R[12]]]]],[11,"hash",E,E,0,[[["self"],["h"]]]],[11,"fmt",E,E,1,[[["self"],[R[13]]],[R[1]]]],[11,"fmt",E,E,2,[[["self"],[R[13]]],[R[1]]]],[11,"fmt",E,E,0,[[["self"],[R[13]]],[R[1]]]],[11,"from",E,E,0,[[["r"]],["self"]]],[11,"next_back",E,E,1,[[["self"]],["option"]]],[11,"len",E,E,1,[[["self"]],["usize"]]],[11,R[10],E,E,0,[[],["iter"]]],[11,"next",E,E,1,[[["self"]],["option"]]],[11,"size_hint",E,E,1,[[["self"]]]],[11,"count",E,E,1,[[],["usize"]]],[11,"clone",E,E,1,[[["self"]],["iter"]]],[11,"clone",E,E,2,[[["self"]],["iter"]]],[11,"clone",E,E,0,[[["self"]],[R[0]]]],[11,R[11],E,E,0,[[]]],[11,"drive_unindexed",E,E,2,[[["c"]]]],[11,"opt_len",E,E,2,[[["self"]],[["option",["usize"]],["usize"]]]]],"p":[[3,R[14]],[3,"Iter"],[3,"ParIter"]]};
initSearch(searchIndex);addSearchOptions(searchIndex);