def read_almanac_file(file_path):
    with open(file_path, "r") as file:
        content = file.read()
    sections = content.strip().split("\n\n")
    _seeds = list(map(int, sections[0].split(":")[1].strip().split()))
    _mappings = [
        parse_map(section.split(":", 1)[1]) for section in sections[1:]
    ]
    return _seeds, _mappings


def parse_map(mapping_str):
    mappings = []
    for line in mapping_str.strip().split("\n"):
        dest_start, src_start, length = map(int, line.split())
        mappings.append((dest_start, src_start, length))
    return mappings


def find_corresponding_number(number, mapping):
    for dest_start, src_start, length in mapping:
        if src_start <= number < src_start + length:
            return dest_start + (number - src_start)
    return number


def process_seed(seed, mappings):
    for mapping in mappings:
        seed = find_corresponding_number(seed, mapping)
    return seed


def part_one(file_path):
    seeds, mappings = read_almanac_file(file_path)
    lowest_location = min(process_seed(seed, mappings) for seed in seeds)
    print(lowest_location)


if __name__ == "__main__":
    part_one("input.txt")
