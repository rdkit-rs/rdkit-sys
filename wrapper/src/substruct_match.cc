#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/Substruct/SubstructMatch.h>

namespace RDKit {
    std::unique_ptr<std::vector<MatchVectType>> substruct_match(std::shared_ptr<ROMol> mol, std::shared_ptr<ROMol> other_mol, std::shared_ptr<SubstructMatchParameters> params) {
        std::vector<MatchVectType> match = SubstructMatch(*mol, *other_mol, *params);
        std::vector<MatchVectType> *heap_match = new std::vector<MatchVectType>(match);
        return std::unique_ptr<std::vector<MatchVectType>>(heap_match);
    }

    bool substruct_match_as_bool(std::shared_ptr<ROMol> mol, std::shared_ptr<ROMol> other_mol, std::shared_ptr<SubstructMatchParameters> params) {
        std::vector<MatchVectType> match = SubstructMatch(*mol, *other_mol, *params);

//        std::cout << "size: " << match.size() << std::endl;
//        auto index = 0;
//        for (auto &submatch: match) {
//            auto pair_index = 0;
//            for(auto &pair: submatch) {
//                std::cout << index << " " << pair_index << " first: " << pair.first << " second: " << pair.second << std::endl;
//                pair_index += 1;
//            }
//            index += 1;
//        }

        return match.size() > 0;
    }

    std::shared_ptr<SubstructMatchParameters> new_substruct_match_parameters() {
        return std::shared_ptr<SubstructMatchParameters>(new SubstructMatchParameters());
    }
}
