var searchIndex = {};
searchIndex["ruroonga_expr"] = {"doc":"# Ruroonga Expr","items":[[0,"expr","ruroonga_expr","",null,null],[4,"Unescaped","ruroonga_expr::expr","",null,null],[4,"Escaped","","",null,null],[11,"eq","","",0,null],[11,"clone","","",0,null],[11,"fmt","","",0,null],[11,"eq","","",1,null],[11,"clone","","",1,null],[11,"fmt","","",1,null],[0,"fulltext_expr","ruroonga_expr","",null,null],[3,"FulltextExpr","ruroonga_expr::fulltext_expr","",null,null],[11,"eq","","",2,null],[11,"ne","","",2,null],[11,"clone","","",2,null],[11,"fmt","","",2,null],[11,"new","","",2,{"inputs":[{"name":"t"}],"output":{"name":"fulltextexpr"}}],[11,"column","","",2,null],[11,"is_invalid","","",2,null],[11,"prepare","","Prepare grn_expr",2,null],[11,"build","","Build grn_expr for fulltext search",2,null],[11,"to_fragment","","",2,null],[11,"add","","Make LogicalAndBuilder with add operation.",2,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",2,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",2,null],[11,"rem","","Make GroupBuilder with rem operation.",2,null],[0,"phrase_expr","ruroonga_expr","",null,null],[3,"PhraseExpr","ruroonga_expr::phrase_expr","",null,null],[11,"eq","","",3,null],[11,"ne","","",3,null],[11,"clone","","",3,null],[11,"fmt","","",3,null],[11,"new","","",3,{"inputs":[{"name":"t"}],"output":{"name":"phraseexpr"}}],[11,"column","","",3,null],[11,"is_invalid","","",3,null],[11,"prepare","","Prepare grn_expr",3,null],[11,"build","","Build grn_expr for phrase search",3,null],[11,"to_fragment","","",3,null],[11,"add","","Make LogicalAndBuilder with add operation.",3,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",3,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",3,null],[11,"rem","","Make GroupBuilder with rem operation.",3,null],[0,"prefix_expr","ruroonga_expr","",null,null],[3,"PrefixExpr","ruroonga_expr::prefix_expr","",null,null],[11,"eq","","",4,null],[11,"ne","","",4,null],[11,"clone","","",4,null],[11,"fmt","","",4,null],[11,"new","","",4,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"prefixexpr"}}],[11,"prepare","","Prepare grn_expr",4,null],[11,"build","","Build grn_expr for prefix search",4,null],[11,"to_fragment","","",4,null],[11,"add","","Make LogicalAndBuilder with add operation.",4,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",4,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",4,null],[11,"rem","","Make GroupBuilder with rem operation.",4,null],[0,"suffix_expr","ruroonga_expr","",null,null],[3,"SuffixExpr","ruroonga_expr::suffix_expr","",null,null],[11,"eq","","",5,null],[11,"ne","","",5,null],[11,"clone","","",5,null],[11,"fmt","","",5,null],[11,"new","","",5,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"suffixexpr"}}],[11,"prepare","","Prepare grn_expr",5,null],[11,"build","","Build grn_expr for suffix search",5,null],[11,"to_fragment","","",5,null],[11,"add","","Make LogicalAndBuilder with add operation.",5,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",5,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",5,null],[11,"rem","","Make GroupBuilder with rem operation.",5,null],[0,"equal_expr","ruroonga_expr","",null,null],[3,"EqualExpr","ruroonga_expr::equal_expr","",null,null],[11,"eq","","",6,null],[11,"ne","","",6,null],[11,"clone","","",6,null],[11,"fmt","","",6,null],[11,"new","","",6,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"equalexpr"}}],[11,"prepare","","Prepare grn_expr",6,null],[11,"build","","Build grn_expr for equal search",6,null],[11,"to_fragment","","",6,null],[11,"add","","Make LogicalAndBuilder with add operation.",6,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",6,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",6,null],[11,"rem","","Make GroupBuilder with rem operation.",6,null],[0,"notequal_expr","ruroonga_expr","",null,null],[3,"NotequalExpr","ruroonga_expr::notequal_expr","",null,null],[11,"eq","","",7,null],[11,"ne","","",7,null],[11,"clone","","",7,null],[11,"fmt","","",7,null],[11,"new","","",7,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"notequalexpr"}}],[11,"prepare","","Prepare grn_expr",7,null],[11,"build","","Build grn_expr for not equal search",7,null],[11,"to_fragment","","",7,null],[11,"add","","Make LogicalAndBuilder with add operation.",7,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",7,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",7,null],[11,"rem","","Make GroupBuilder with rem operation.",7,null],[0,"less_expr","ruroonga_expr","",null,null],[3,"LessExpr","ruroonga_expr::less_expr","",null,null],[11,"eq","","",8,null],[11,"ne","","",8,null],[11,"clone","","",8,null],[11,"fmt","","",8,null],[11,"new","","",8,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"lessexpr"}}],[11,"prepare","","Prepare grn_expr",8,null],[11,"build","","Build grn_expr for less search",8,null],[11,"to_fragment","","",8,null],[11,"add","","Make LogicalAndBuilder with add operation.",8,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",8,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",8,null],[11,"rem","","Make GroupBuilder with rem operation.",8,null],[0,"greater_expr","ruroonga_expr","",null,null],[3,"GreaterExpr","ruroonga_expr::greater_expr","",null,null],[11,"eq","","",9,null],[11,"ne","","",9,null],[11,"clone","","",9,null],[11,"fmt","","",9,null],[11,"new","","",9,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"greaterexpr"}}],[11,"prepare","","Prepare grn_expr",9,null],[11,"build","","Build grn_expr for greater search",9,null],[11,"to_fragment","","",9,null],[11,"add","","Make LogicalAndBuilder with add operation.",9,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",9,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",9,null],[11,"rem","","Make GroupBuilder with rem operation.",9,null],[0,"less_equal_expr","ruroonga_expr","",null,null],[3,"LessEqualExpr","ruroonga_expr::less_equal_expr","",null,null],[11,"eq","","",10,null],[11,"ne","","",10,null],[11,"clone","","",10,null],[11,"fmt","","",10,null],[11,"new","","",10,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"lessequalexpr"}}],[11,"prepare","","Prepare grn_expr",10,null],[11,"build","","Build grn_expr for less equal search",10,null],[11,"to_fragment","","",10,null],[11,"add","","Make LogicalAndBuilder with add operation.",10,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",10,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",10,null],[11,"rem","","Make GroupBuilder with rem operation.",10,null],[0,"greater_equal_expr","ruroonga_expr","",null,null],[3,"GreaterEqualExpr","ruroonga_expr::greater_equal_expr","",null,null],[11,"eq","","",11,null],[11,"ne","","",11,null],[11,"clone","","",11,null],[11,"fmt","","",11,null],[11,"new","","",11,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"greaterequalexpr"}}],[11,"prepare","","Prepare grn_expr",11,null],[11,"build","","Build grn_expr for greater equal search",11,null],[11,"to_fragment","","",11,null],[11,"add","","Make LogicalAndBuilder with add operation.",11,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",11,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",11,null],[11,"rem","","Make GroupBuilder with rem operation.",11,null],[0,"match_expr","ruroonga_expr","",null,null],[3,"MatchExpr","ruroonga_expr::match_expr","",null,null],[11,"eq","","",12,null],[11,"ne","","",12,null],[11,"clone","","",12,null],[11,"fmt","","",12,null],[11,"new","","",12,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"matchexpr"}}],[11,"prepare","","Prepare grn_expr",12,null],[11,"build","","Build grn_expr for greater search",12,null],[11,"to_fragment","","",12,null],[11,"add","","Make LogicalAndBuilder with add operation.",12,null],[11,"sub","","Make LogicalNotBuilder with sub operation.",12,null],[11,"bitor","","Make LogicalOrBuilder with bitor operation.",12,null],[11,"rem","","Make GroupBuilder with rem operation.",12,null],[0,"groupable","ruroonga_expr","",null,null],[0,"logical_and_builder","ruroonga_expr::groupable","Create logical And Groonga expression.",null,null],[3,"LogicalAndBuilder","ruroonga_expr::groupable::logical_and_builder","",null,null],[11,"eq","","",13,null],[11,"ne","","",13,null],[11,"clone","","",13,null],[11,"fmt","","",13,null],[11,"new","","",13,{"inputs":[{"name":"query"},{"name":"query"}],"output":{"name":"logicalandbuilder"}}],[11,"build","","",13,null],[11,"to_fragment","","",13,null],[11,"to_group","","",13,null],[0,"logical_or_builder","ruroonga_expr::groupable","Create logical Or Groonga expression.",null,null],[3,"LogicalOrBuilder","ruroonga_expr::groupable::logical_or_builder","",null,null],[11,"eq","","",14,null],[11,"ne","","",14,null],[11,"clone","","",14,null],[11,"fmt","","",14,null],[11,"new","","",14,{"inputs":[{"name":"query"},{"name":"query"}],"output":{"name":"logicalorbuilder"}}],[11,"build","","",14,null],[11,"to_fragment","","",14,null],[11,"to_group","","",14,null],[0,"logical_not_builder","ruroonga_expr::groupable","Create logical Not Groonga expression.",null,null],[3,"LogicalNotBuilder","ruroonga_expr::groupable::logical_not_builder","",null,null],[11,"eq","","",15,null],[11,"ne","","",15,null],[11,"clone","","",15,null],[11,"fmt","","",15,null],[11,"new","","",15,{"inputs":[{"name":"query"},{"name":"query"}],"output":{"name":"logicalnotbuilder"}}],[11,"build","","",15,null],[11,"to_fragment","","",15,null],[11,"to_group","","",15,null],[0,"group_builder","ruroonga_expr::groupable","Create grouping Groonga expression.",null,null],[3,"GroupBuilder","ruroonga_expr::groupable::group_builder","",null,null],[11,"eq","","",16,null],[11,"ne","","",16,null],[11,"clone","","",16,null],[11,"fmt","","",16,null],[11,"new","","",16,{"inputs":[{"name":"query"},{"name":"query"}],"output":{"name":"groupbuilder"}}],[11,"build","","",16,null],[11,"to_fragment","","",16,null],[11,"to_group","","",16,null],[6,"Query","ruroonga_expr::groupable","",null,null],[8,"Fragmentable","","",null,null],[10,"to_fragment","","Make groonga expression fragment.",17,null],[8,"Groupable","","",null,null],[10,"to_group","","Make groupable groonga expression fragment.",18,null],[0,"dsl","ruroonga_expr","",null,null],[5,"fulltext_expr","ruroonga_expr::dsl","",null,{"inputs":[{"name":"t"}],"output":{"name":"fulltextexpr"}}],[5,"phrase_expr","","",null,{"inputs":[{"name":"t"}],"output":{"name":"phraseexpr"}}],[5,"prefix_expr","","",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"prefixexpr"}}],[5,"suffix_expr","","",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"suffixexpr"}}],[5,"equal_expr","","",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"equalexpr"}}],[5,"notequal_expr","","",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"notequalexpr"}}],[5,"less_expr","","",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"lessexpr"}}],[5,"greater_expr","","",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"greaterexpr"}}],[5,"less_equal_expr","","",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"lessequalexpr"}}],[5,"greater_equal_expr","","",null,{"inputs":[{"name":"t"},{"name":"t"}],"output":{"name":"greaterequalexpr"}}],[0,"logical","","",null,null],[5,"and","ruroonga_expr::dsl::logical","",null,{"inputs":[{"name":"query"},{"name":"query"}],"output":{"name":"logicalandbuilder"}}],[5,"not","","",null,{"inputs":[{"name":"query"},{"name":"query"}],"output":{"name":"logicalnotbuilder"}}],[5,"or","","",null,{"inputs":[{"name":"query"},{"name":"query"}],"output":{"name":"logicalorbuilder"}}],[5,"group","","",null,{"inputs":[{"name":"query"},{"name":"query"}],"output":{"name":"groupbuilder"}}]],"paths":[[4,"Unescaped"],[4,"Escaped"],[3,"FulltextExpr"],[3,"PhraseExpr"],[3,"PrefixExpr"],[3,"SuffixExpr"],[3,"EqualExpr"],[3,"NotequalExpr"],[3,"LessExpr"],[3,"GreaterExpr"],[3,"LessEqualExpr"],[3,"GreaterEqualExpr"],[3,"MatchExpr"],[3,"LogicalAndBuilder"],[3,"LogicalOrBuilder"],[3,"LogicalNotBuilder"],[3,"GroupBuilder"],[8,"Fragmentable"],[8,"Groupable"]]};
initSearch(searchIndex);