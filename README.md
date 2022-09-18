# Perceptron
Artificial Neural Networks in Rust 

This is one of the first artificial neural network implementation in pure Rust written from scratch. The gif in the example is the visualization of the self-learning algorithm of binary classifiers. What you are seeing is 62500 neurons trying to learn spacial and rotational patterns in order to identify randomly generated circles and rectangles. In each frame, any given neuron updates its current state to match the desired output. This algorithm is guaranteed to converge so long as it is possible to map the two categories into distinct groups.
