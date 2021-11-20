import logging

__logger = logging.getLogger("evet")


def set_log_level(verbosity: int = 0) -> None:
    if verbosity >= 3:
        level = logging.DEBUG
    elif verbosity == 2:
        level = logging.INFO
    elif verbosity == 1:
        level = logging.ERROR
    else:
        level = logging.FATAL
    logging.basicConfig(level=level)


def get_logger() -> logging.Logger:
    return __logger
