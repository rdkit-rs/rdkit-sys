#pragma once

#include "rust/cxx.h"

namespace RDKit {
    std::shared_ptr<RWMol> rw_mol_from_ro_mol(std::shared_ptr<ROMol> mol, bool quick_copy, int conf_id);
}