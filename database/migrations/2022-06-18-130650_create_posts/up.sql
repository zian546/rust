CREATE TABLE `api_case_project`.`USER` (
    `id` INT NOT NULL,
    `username` VARCHAR(255) NOT NULL,
    `password` VARCHAR(255) NOT NULL,
    `value` INT NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE (`username`)
) ENGINE = InnoDB;