export function toPascalCase(text: string): string {
  return text
    ?.split("_")
    .map((word) =>
      word.replace(
        /(\w)(\w*)/g,
        (_, firstLetter, remainingLetters) =>
          firstLetter.toUpperCase() + remainingLetters.toLowerCase()
      )
    )
    .join("");
}