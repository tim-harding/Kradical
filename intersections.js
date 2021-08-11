const fs = require("fs")

function main() {
    const krad_json = fs.readFileSync("./assets/outputs/krad.json")
    const krad = JSON.parse(krad_json)
    const radicals = new Set()
    for (const entry of krad) {
        for (const radical of entry.radicals) {
            radicals.add(radical)
        }
    }
    const intersections = []
    for (const radical of radicals) {
        const intersection = {
            radical,
            adjacents: []
        }
        for (const entry of krad) {
            if (entry.radicals.includes(radical)) {
                for (const adjacent of entry.radicals) {
                    if (adjacent !== radical && !intersection.adjacents.includes(adjacent)) {
                        intersection.adjacents.push(adjacent)
                    }
                }
            }
        }
        intersections.push(intersection)
    }
    const intersections_json = JSON.stringify(intersections, null, 2)
    fs.writeFileSync("./assets/outputs/intersections.json", intersections_json)
}

main()