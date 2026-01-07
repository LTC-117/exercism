public class Lasagna {
    private int expectedMinutes = 40;
    private int layerTime = 2;

    public int expectedMinutesInOven() {
        return this.expectedMinutes;
    }

    public int remainingMinutesInOven(int minutesPassed) {
        return expectedMinutesInOven() - minutesPassed;
    }

    public int preparationTimeInMinutes(int numOfLayers) {
        return numOfLayers * this.layerTime;
    }

    public int totalTimeInMinutes(int numOfLayers, int minutesPassed) {
        return minutesPassed + (preparationTimeInMinutes(numOfLayers));
    }
}
