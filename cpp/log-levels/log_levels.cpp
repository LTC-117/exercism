#include <string>

namespace log_line {
    std::string message(std::string line) {
        std::string message;

        int index = line.find(":");
        message = line.substr(index + 2);

        return message;
    }

    std::string log_level(std::string line) {
        std::string log_level;

        int end = line.find("]");
        log_level = line.substr(1, end - 1);

        return log_level;
    }

    std::string reformat(std::string line) {
        std::string reformat;

        std::string message = log_line::message(line);
        std::string log_level = log_line::log_level(line);

        reformat = message + " (" + log_level + ")";

        return reformat;
    }
}  // namespace log_line
