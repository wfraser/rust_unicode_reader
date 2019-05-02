var N=null,E="",T="t",U="u",searchIndex={};
var R=["unicode_segmentation","as_str","View the underlying data (the part yet to be iterated) as…","usize","result","graphemeincomplete","option","graphemes","into_iter","try_from","try_into","borrow_mut","type_id","borrow","typeid","next_back","size_hint","graphemeindices","graphemecursor","uwordbounds","uwordboundindices","GraphemeIncomplete","GraphemeIndices","Graphemes","GraphemeCursor","UWordBoundIndices","UWordBounds","UnicodeSegmentation","UnicodeWords","unicode_reader","codepoints","formatter","BadUtf8Error","CodePoints"];
searchIndex[R[29]]={"doc":"This crate provides adaptors which wrap byte-oriented…","i":[[3,R[33],R[29],"Wraps a byte-oriented reader and yields the UTF-8 data one…",N,N],[3,R[32],E,"An error raised when parsing a UTF-8 byte stream fails.",N,N],[12,"bytes",E,"The bytes that could not be parsed as a code point.",0,N],[3,R[23],E,"Wraps a `char`-oriented reader and yields the data one…",N,N],[11,R[8],E,E,1,[[["self"]],["i"]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[9],E,E,1,[[[U]],[R[4]]]],[11,R[13],E,E,1,[[["self"]],[T]]],[11,R[12],E,E,1,[[["self"]],[R[14]]]],[11,R[11],E,E,1,[[["self"]],[T]]],[11,R[10],E,E,1,[[["self"]],[R[4]]]],[11,"to_string",E,E,0,[[["self"]],["string"]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[9],E,E,0,[[[U]],[R[4]]]],[11,R[13],E,E,0,[[["self"]],[T]]],[11,R[12],E,E,0,[[["self"]],[R[14]]]],[11,R[11],E,E,0,[[["self"]],[T]]],[11,R[10],E,E,0,[[["self"]],[R[4]]]],[11,R[8],E,E,2,[[["self"]],["i"]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[["self"]],[U]]],[11,R[9],E,E,2,[[[U]],[R[4]]]],[11,R[13],E,E,2,[[["self"]],[T]]],[11,R[12],E,E,2,[[["self"]],[R[14]]]],[11,R[11],E,E,2,[[["self"]],[T]]],[11,R[10],E,E,2,[[["self"]],[R[4]]]],[11,"next",E,"Get the next Unicode code point from the stream. Any…",1,[[["self"]],[R[6]]]],[11,"next",E,"Get the next grapheme cluster from the stream. Note that…",2,[[["self"]],[R[6]]]],[11,"from",E,E,1,[[["r"]],[R[30]]]],[11,"from",E,E,2,[[["r"]],[R[7]]]],[11,"from",E,E,1,[[["r"]],[R[30],["bytes"]]]],[11,"from",E,E,2,[[["r"]],[R[7],[R[30]]]]],[11,"fmt",E,E,0,[[["self"],[R[31]]],[R[4]]]],[11,"fmt",E,E,0,[[["self"],[R[31]]],[R[4]]]],[11,"description",E,E,0,[[["self"]],["str"]]]],"p":[[3,R[32]],[3,R[33]],[3,R[23]]]};
searchIndex[R[0]]={"doc":"Iterators which split strings on Grapheme Cluster or Word…","i":[[3,R[23],R[0],"External iterator for a string's grapheme clusters.",N,N],[3,R[22],E,"External iterator for grapheme clusters and byte offsets.",N,N],[3,R[24],E,"Cursor-based segmenter for grapheme clusters.",N,N],[3,R[26],E,"External iterator for a string's word boundaries.",N,N],[3,R[25],E,"External iterator for word boundaries and byte offsets.",N,N],[3,R[28],E,"An iterator over the substrings of a string which, after…",N,N],[4,R[21],E,"An error return indicating that not enough content was…",N,N],[13,"PreContext",E,"More pre-context is needed. The caller should call…",0,N],[13,"PrevChunk",E,"When requesting `prev_boundary`, the cursor is moving past…",0,N],[13,"NextChunk",E,"When requesting `next_boundary`, the cursor is moving past…",0,N],[13,"InvalidOffset",E,"An error returned when the chunk given does not contain…",0,N],[11,R[1],E,R[2],1,[[["self"]],["str"]]],[11,R[1],E,R[2],2,[[["self"]],["str"]]],[11,"new",E,"Create a new cursor. The string and initial offset are…",3,[[[R[3]],[R[3]],["bool"]],[R[18]]]],[11,"set_cursor",E,"Set the cursor to a new location in the same string.",3,[[["self"],[R[3]]]]],[11,"cur_cursor",E,"The current offset of the cursor. Equal to the last value…",3,[[["self"]],[R[3]]]],[11,"provide_context",E,"Provide additional pre-context when it is needed to decide…",3,[[["self"],["str"],[R[3]]]]],[11,"is_boundary",E,"Determine whether the current cursor location is a…",3,[[["self"],["str"],[R[3]]],[R[4],["bool",R[5]]]]],[11,"next_boundary",E,"Find the next boundary after the current cursor position.…",3,[[["self"],["str"],[R[3]]],[R[4],[R[6],R[5]]]]],[11,"prev_boundary",E,"Find the previous boundary after the current cursor…",3,[[["self"],["str"],[R[3]]],[R[4],[R[6],R[5]]]]],[11,R[1],E,R[2],4,[[["self"]],["str"]]],[11,R[1],E,R[2],5,[[["self"]],["str"]]],[17,"UNICODE_VERSION",E,"The version of Unicode that this version of…",N,N],[8,R[27],E,"Methods for segmenting strings according to Unicode…",N,N],[10,R[7],E,"Returns an iterator over the [grapheme…",6,[[["self"],["bool"]],[R[7]]]],[10,"grapheme_indices",E,"Returns an iterator over the grapheme clusters of `self`…",6,[[["self"],["bool"]],[R[17]]]],[10,"unicode_words",E,"Returns an iterator over the words of `self`, separated on…",6,[[["self"]],["unicodewords"]]],[10,"split_word_bounds",E,"Returns an iterator over substrings of `self` separated on…",6,[[["self"]],[R[19]]]],[10,"split_word_bound_indices",E,"Returns an iterator over substrings of `self`, split on…",6,[[["self"]],[R[20]]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[8],E,E,2,[[["self"]],["i"]]],[11,R[9],E,E,2,[[[U]],[R[4]]]],[11,R[10],E,E,2,[[["self"]],[R[4]]]],[11,"into",E,E,2,[[["self"]],[U]]],[11,R[13],E,E,2,[[["self"]],[T]]],[11,R[11],E,E,2,[[["self"]],[T]]],[11,R[12],E,E,2,[[["self"]],[R[14]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[8],E,E,1,[[["self"]],["i"]]],[11,R[9],E,E,1,[[[U]],[R[4]]]],[11,R[10],E,E,1,[[["self"]],[R[4]]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[13],E,E,1,[[["self"]],[T]]],[11,R[11],E,E,1,[[["self"]],[T]]],[11,R[12],E,E,1,[[["self"]],[R[14]]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[9],E,E,3,[[[U]],[R[4]]]],[11,R[10],E,E,3,[[["self"]],[R[4]]]],[11,"into",E,E,3,[[["self"]],[U]]],[11,R[13],E,E,3,[[["self"]],[T]]],[11,R[11],E,E,3,[[["self"]],[T]]],[11,R[12],E,E,3,[[["self"]],[R[14]]]],[11,"from",E,E,5,[[[T]],[T]]],[11,R[8],E,E,5,[[["self"]],["i"]]],[11,R[9],E,E,5,[[[U]],[R[4]]]],[11,R[10],E,E,5,[[["self"]],[R[4]]]],[11,"into",E,E,5,[[["self"]],[U]]],[11,R[13],E,E,5,[[["self"]],[T]]],[11,R[11],E,E,5,[[["self"]],[T]]],[11,R[12],E,E,5,[[["self"]],[R[14]]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[8],E,E,4,[[["self"]],["i"]]],[11,R[9],E,E,4,[[[U]],[R[4]]]],[11,R[10],E,E,4,[[["self"]],[R[4]]]],[11,"into",E,E,4,[[["self"]],[U]]],[11,R[13],E,E,4,[[["self"]],[T]]],[11,R[11],E,E,4,[[["self"]],[T]]],[11,R[12],E,E,4,[[["self"]],[R[14]]]],[11,"from",E,E,7,[[[T]],[T]]],[11,R[8],E,E,7,[[["self"]],["i"]]],[11,R[9],E,E,7,[[[U]],[R[4]]]],[11,R[10],E,E,7,[[["self"]],[R[4]]]],[11,"into",E,E,7,[[["self"]],[U]]],[11,R[13],E,E,7,[[["self"]],[T]]],[11,R[11],E,E,7,[[["self"]],[T]]],[11,R[12],E,E,7,[[["self"]],[R[14]]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[9],E,E,0,[[[U]],[R[4]]]],[11,R[10],E,E,0,[[["self"]],[R[4]]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[13],E,E,0,[[["self"]],[T]]],[11,R[11],E,E,0,[[["self"]],[T]]],[11,R[12],E,E,0,[[["self"]],[R[14]]]],[11,"fmt",E,E,0,[[["self"],[R[31]]],[R[4]]]],[11,"eq",E,E,0,[[["self"],[R[5]]],["bool"]]],[11,"ne",E,E,0,[[["self"],[R[5]]],["bool"]]],[11,R[15],E,E,1,[[["self"]],[R[6]]]],[11,R[15],E,E,2,[[["self"]],[R[6],["str"]]]],[11,R[15],E,E,7,[[["self"]],[R[6],["str"]]]],[11,R[15],E,E,4,[[["self"]],[R[6]]]],[11,R[15],E,E,5,[[["self"]],[R[6],["str"]]]],[11,"next",E,E,1,[[["self"]],[R[6]]]],[11,R[16],E,E,1,N],[11,R[16],E,E,2,N],[11,"next",E,E,2,[[["self"]],[R[6],["str"]]]],[11,"next",E,E,7,[[["self"]],[R[6],["str"]]]],[11,"next",E,E,4,[[["self"]],[R[6]]]],[11,R[16],E,E,4,N],[11,R[16],E,E,5,N],[11,"next",E,E,5,[[["self"]],[R[6],["str"]]]],[11,"clone",E,E,1,[[["self"]],[R[17]]]],[11,"clone",E,E,2,[[["self"]],[R[7]]]],[11,"clone",E,E,3,[[["self"]],[R[18]]]],[11,"clone",E,E,5,[[["self"]],[R[19]]]],[11,"clone",E,E,4,[[["self"]],[R[20]]]]],"p":[[4,R[21]],[3,R[22]],[3,R[23]],[3,R[24]],[3,R[25]],[3,R[26]],[8,R[27]],[3,R[28]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);