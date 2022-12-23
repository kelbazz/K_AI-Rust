/* * * * * * * * * * * * * * * * * * * * * * * * * * *\
* v1.4   Kelbaz Artificial Intellingence    By Kelbaz *
*                                                     *
*        @@@@@@@@@@@@@@   @@,              @*`'@      *
*        @@@@@@@@@@@@'    @@@@,            @. ,@      *
*     ,@@ `@@@@@@@@'      @'  `@,          @@@@@      *
*     @@@  :@@@@@'        @.  ,@@@,        @@@@@      *
*     `@@ ,@@@@' @@,      @@@@@`@@@@,      @@@@@      *
*        @@@@'   @@@@,    @@@@@ `@@@@@,    @@@@@      *
*        @@'     @@@@@@   @@@@@   `@@@@@   @@@@@      *
*                                                     *
\* * * * * * * * * * * * * * * * * * * * * * * * * * */

/// The module containing the Activation struct and basics ones.
pub mod activations;
/// The matrix module powering the Neural Network system.
mod matrix;
/// The actual Neural Network module containing the `Network` and `DatasetValue` structs.
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn nn_test() {
        use super::{
            activations::SIGMOID,
            network::{DatasetValue, Network},
        };

        let dataset: &[DatasetValue] = &[
            DatasetValue {
                inputs: &[0.0, 0.0],
                targets: &[0.0],
            },
            DatasetValue {
                inputs: &[0.0, 1.0],
                targets: &[1.0],
            },
            DatasetValue {
                inputs: &[1.0, 0.0],
                targets: &[1.0],
            },
            DatasetValue {
                inputs: &[1.0, 1.0],
                targets: &[0.0],
            },
        ];

        let mut nn = Network::new(&[2, 4, 1], 0.1, SIGMOID);

        nn.train(&dataset, 10_000);
        for i in 0..dataset.len() {
            let res: Vec<f64> = nn.feed_forward(dataset[i].inputs).unwrap();
            println!(
                "{:?}\n => {:?}\n \\=> {}",
                dataset[i].inputs,
                res,
                res[0] > 0.5
            );
        }
    }
}
