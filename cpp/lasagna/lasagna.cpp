// ovenTime returns the amount in minutes that the lasagna should stay in the
// oven.
int ovenTime() {
    return 40;
}

/* remainingOvenTime returns the remaining
   minutes based on the actual minutes already in the oven.
*/
int remainingOvenTime(int actualMinutesInOven) {
    int remaining_time = ovenTime() - actualMinutesInOven;

    return remaining_time;
}

/* preparationTime returns an estimate of the preparation time based on the
   number of layers and the necessary time per layer.
*/
int preparationTime(int numberOfLayers) {
    int time_per_layer = 2;

    return numberOfLayers * time_per_layer;;
}

// elapsedTime calculates the total time spent to create and bake the lasagna so
// far.
int elapsedTime(int numberOfLayers, int actualMinutesInOven) {
    int preparation_time = preparationTime(numberOfLayers);

    return preparation_time + actualMinutesInOven;
}
