#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>

namespace RDKit {
  using RemoveHsParameters = RDKit::MolOps::RemoveHsParameters;
  std::shared_ptr<RemoveHsParameters> new_remove_hs_parameters() {
    return std::shared_ptr<RemoveHsParameters>(new RemoveHsParameters);
  }

  std::shared_ptr<ROMol> remove_hs_parameters(
      const std::shared_ptr<ROMol> &mol,
      const std::shared_ptr<RemoveHsParameters> &params,
      bool sanitize
  ) {
    ROMol *new_mol = MolOps::removeHs(*mol, *params, sanitize);
    return std::shared_ptr<ROMol>(new_mol);
  }
}