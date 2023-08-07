// #[cfg(test)]
use crate::function::Function;
use crate::guard::Guard;
use crate::signature::Signature;
use itertools::Itertools;
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A struct representing transitions between places in a Petri net.
/// 
/// The transition encodes the input and output signatures of the transition's tokens
/// (e.g. what is consumed and produced), as well as a guard that further specifies what tokens
/// are allowed to be consumed by the transition. The guard defines a set of addresses and qualifiers
/// that must be satisfied by the tokens in order for the transition to fire, and evaluates to a
/// boolean when given a set of tokens. The transition also encodes a function that specifies the
/// output of the transition given the input, mapping input signatures to output assignments, as 
/// well as a time and cost for the transition.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Transition {
    pub id: Uuid,
    pub name: String,
    // Defines the variables that are consumed by the transition, hashed by the id of the source place
    pub input: HashMap<Uuid, Signature>,
    // Defines the variables thate are produced by the transition, hashed by the id of the target place
    pub output: HashMap<Uuid, Signature>,
    // Guards are indexed by the place id and the index of the clade in the signature
    pub guard: Guard,
    // Specifies the function result at each outgoing edge, hashed by the id of the target place
    pub function: Function,
}

impl Transition {
    pub fn new(
        name: String,
        input: Option<HashMap<Uuid, Signature>>,
        output: Option<HashMap<Uuid, Signature>>,
        guard: Option<Guard>,
        function: Option<Function>,
    ) -> Self {
        let mut final_input: HashMap<Uuid,Signature> = HashMap::new();
        let mut final_output: HashMap<Uuid,Signature> = HashMap::new();
        let mut final_guard: Guard = Guard::Empty;
        let mut final_function: Function = Function::default();

        match (input, guard) {
            (Some(i), Some(g)) => {
                if !g.symbols().iter().all(|symbol| {
                    i.values().flat_map(|signature| signature.symbols.clone()).contains(symbol)
                }) {
                    warn!("Transition {} has guard symbols not present in the input signature. Using an empty guard.", name);
                    final_input = i;
                    
                } else {
                    final_input = i;
                    final_guard = g;
                }
            }
            (Some(i), None) => {
                final_input = i;
            }
            (None, Some(_g)) => {
                warn!(
                    "Transition {} has guard but no input symbols. Using an empty guard.",
                    name
                );
            }
            (None, None) => {}
        }

        match (output,function) {
            (Some(o), Some(f)) => {
                // if !f.get_domain().iter().all(|output_signature| )
                final_output = o;
                final_function = f;
                // if o.values().all(|output_signature| f.get_domain().contains(&output_signature)) {
                //     final_output = o;
                //     final_function = f;
                // } else {
                //     warn!("Transition {} has output signature that is not in the function domain. Using an empty function.", name);
                //     final_output = o;
                // }
                // if !f.get_domain().iter().all(|input_signature| {
                //     o.contains_key(*input_signature)
                // }) {
                //     warn!("Transition {} has function that references an address not in the output signature. Using an empty function.", name);
                //     final_output = o;
                // } else {
                //     final_output = o;
                //     final_function = f;
                // }
            }
            (Some(o), None) => {
                final_output = o;
            }
            (None, Some(_f)) => {
                warn!(
                    "Transition {} has function but no output signature. Using an empty function.",
                    name
                );
            }
            (None, None) => {}
        }
        
        return Self {
            id: Uuid::new_v4(),
            name,
            input: final_input,
            output: final_output,
            guard: final_guard,
            function: final_function,
        };
    }

    // pub fn get_time(&self, signature: &Signature) -> Time {
    //     self.function.get_time(signature).map(|t| *t).unwrap_or(0)
    // }

    // pub fn verify_signature(signature: &Signature, tokens: &Vec<&Token>) -> bool {
    //     if signature.symbols.len() > tokens.len() {
    //         return false;
    //     } else {
    //         return tokens
    //             .iter()
    //             .permutations(tokens.len())
    //             .unique()
    //             .any(|permutation| {
    //                 permutation
    //                     .iter()
    //                     .zip(signature.symbols.iter())
    //                     .all(|(token, clade)| token.clade <= *clade)
    //             });
    //     }
    // }

    // pub fn get_binding(&self, tokens: &HashMap<Uuid, &Vec<&Token>>) -> Option<HashMap<Address,Token>> {
    //     if self.input
    //         .iter()
    //         .all(|(input_id, signature)| 
    //             Self::verify_signature(
    //                 signature, 
    //                 tokens.get(input_id).unwrap_or(&&vec![])
    //             )
    //         ) {
    //             // We know that there should be at least one binding of the input signatures
    //             // Create a set of all possible bindings of the input signatures

                
    //         return None;//self.guard.eval(tokens);
    //     } else {
    //         return None;
    //     }
    // }
}

#[test]
pub fn verify_signature() {
    // let grandchild1 = Clade::new("grandchild1".into(), None);
    // let grandchild2 = Clade::new("grandchild2".into(), None);
    // let child = Clade::new(
    //     "child".into(),
    //     Some(vec![grandchild1.clone(), grandchild2.clone()]),
    // );

    // let sig1 = Signature::new(vec![child.clone(), grandchild2.clone()]);
    // let sig2 = Signature::new(vec![grandchild1.clone(), grandchild2.clone()]);
    // let sig3 = Signature::new(vec![grandchild1.clone(), grandchild2.clone()]);
    // let sig4 = Signature::new(vec![grandchild1.clone()]);
    // let sig5 = Signature::new(vec![child.clone()]);

    // let token1 = Token::new("child".into(), child.clone());
    // let token2 = Token::new("grandchild1".into(), grandchild1.clone());
    // let token3 = Token::new("grandchild2".into(), grandchild2.clone());

    // assert!(Transition::verify_signature(
    //     &sig1,
    //     &vec![&token1, &token2, &token3]
    // ));
    // assert!(!Transition::verify_signature(
    //     &sig2,
    //     &vec![&token2, &token2]
    // ));
    // assert!(Transition::verify_signature(
    //     &sig3,
    //     &vec![&token2, &token3]
    // ));
    // assert!(Transition::verify_signature(
    //     &sig3,
    //     &vec![&token3, &token2]
    // ));
    // assert!(!Transition::verify_signature(
    //     &sig4,
    //     &vec![&token1]
    // ));
    // assert!(Transition::verify_signature(
    //     &sig5,
    //     &vec![&token2]
    // ));
}

#[test]
pub fn verify_guard() {

}
