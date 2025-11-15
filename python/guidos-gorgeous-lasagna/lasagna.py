"""Functions used in preparing Guido's gorgeous lasagna.

Learn about Guido, the creator of the Python language:
https://en.wikipedia.org/wiki/Guido_van_Rossum

This is a module docstring, used to describe the functionality
of a module and its functions and/or classes.
"""


EXPECTED_BAKE_TIME: int = 40
PREPARATION_TIME: int = 2


def bake_time_remaining(elapsed_bake_time: int):
    """Calculate the bake time remaining.

    :param elapsed_bake_time: int - baking time already elapsed.
    :return: int - remaining bake time (in minutes) derived from 'EXPECTED_BAKE_TIME'.

    Function that takes the actual minutes the lasagna has been in the oven as
    an argument and returns how many minutes the lasagna still needs to bake
    based on the `EXPECTED_BAKE_TIME`.
    """

    remaining: int = EXPECTED_BAKE_TIME - elapsed_bake_time

    return remaining


def preparation_time_in_minutes(number_of_layers: int):
    """Calculate the preparation time in minutes.

    :param number_of_layers: int - represents the amount of layers of the lasagna.
    :return: int - preparation time in minutes.

    Function that takes the amount of layers the lasagna is gonna have as an argument,
    and returns the total preparation time of the lasagna based on the argument and the
    amount of time (in minutes) each layer takes to be prepared (PREPARATION_TIME = 2).
    """

    preparation_time: int = number_of_layers * PREPARATION_TIME
    return preparation_time



def elapsed_time_in_minutes(number_of_layers: int, baking_time: int):
    """Calculate total elapsed cooking time (prep + bake) in minutes.

    :param number_of_layers: int - represents the amount of layers of the lasagna.
    :param baking_time: int - represents how much time the lasagna has been baking.
    :return: int - total time elapsed.

    Function that takes both the amount of layers and the amount of time the
    lasagna has been baking and returns the total amount of time passed so far.
    """

    preparation_time: int = preparation_time_in_minutes(number_of_layers)
    elapsed_time: int = preparation_time + baking_time

    return elapsed_time
