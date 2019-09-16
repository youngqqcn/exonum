// Copyright 2019 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The set of specific for the Rust runtime implementation errors.

/// List of possible Rust runtime errors.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoExecutionError)]
#[exonum(crate = "crate", kind = "runtime")]
pub enum Error {
    /// Unable to parse artifact identifier or specified artifact has non-empty spec.
    IncorrectArtifactId = 0,
    /// Unable to deploy artifact with the specified identifier, it is not listed in available artifacts.
    UnableToDeploy = 1,
    /// Unable to parse service configuration.
    ConfigParseError = 2,
    /// Unspecified error during the call invocation.
    UnspecifiedError = 3,
    /// Unable to parse the method arguments of the service interface.
    ArgumentsParseError = 4,
}