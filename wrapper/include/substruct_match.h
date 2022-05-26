#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/Substruct/SubstructMatch.h>

namespace RDKit {
    std::unique_ptr<std::vector<MatchVectType>> substruct_match(std::shared_ptr<ROMol> mol, std::shared_ptr<ROMol> other_mol, std::shared_ptr<SubstructMatchParameters> params);

    bool substruct_match_as_bool(std::shared_ptr<ROMol> mol, std::shared_ptr<ROMol> other_mol, std::shared_ptr<SubstructMatchParameters> params);

    std::shared_ptr<SubstructMatchParameters> new_substruct_match_parameters();
}