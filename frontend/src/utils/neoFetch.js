export const neoFetch = async (path) => {
    let result;
    const response = await fetch(path);

    if (response.ok) {
        const data = await response.json();
        console.log(data);
        result = data;
    } else {
        console.error("Error");
    }

    return result;
}