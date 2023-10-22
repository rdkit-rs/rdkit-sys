#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>


namespace RDKit {
//  pub fn new_remove_hs_parameters() -> SharedPtr<RemoveHsParameters>;
  using RemoveHsParameters = RDKit::MolOps::RemoveHsParameters;
  std::shared_ptr<RemoveHsParameters> new_remove_hs_parameters();

  std::shared_ptr<ROMol> remove_hs_parameters(
      const std::shared_ptr<ROMol> &mol,
      const std::shared_ptr<RemoveHsParameters> &params,
      bool sanitize
  );
//
//  pub fn add_hs(
//      mol: &SharedPtr<ROMol>,
//      explicit_only: bool,
//      add_coords: bool,
//      only_on_atoms: &Vec<u32>,
//      add_residue_info: bool,
//  );
}
