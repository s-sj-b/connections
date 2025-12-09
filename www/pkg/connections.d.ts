/* tslint:disable */
/* eslint-disable */
export function greet(s: string): void;
export function remove_element_by_id(id: string): void;
export enum AnswerGroup {
  Yellow = 0,
  Green = 1,
  Blue = 2,
  Purple = 3,
}
export enum SquareState {
  Plain = 0,
  Selected = 1,
  Answered = 2,
}
export class Answer {
  private constructor();
  free(): void;
  static new(group: string, description: string, values: string[]): Answer;
}
/**
 * Handles the state of the AnswerGrid and interoperability with the JavaScript.
 * 
 * Since `selected` stores positions of the selected squares within the `squares` vector,
 * `width` and `height` are needed to calculate from the from the grid row and column.
 * 
 * `lives` tracks how many incorrect guesses have been submitted.
 */
export class AnswerGrid {
  private constructor();
  free(): void;
  /**
   * Returns a `String` of `n` hearts, where `n` is the number of lives remaining.
   */
  lives_string(): string;
  toggle_square(x: number, y: number): void;
  /**
   * Submits the selected squares for review.
   * 
   * If the selection is invalid (fewer than 4 were chosen), returns `None`. 
   * If the selection was valid, returns `Some(true)` if the selection was correct, or `Some(false)` if it was not.
   */
  submit_selection(): boolean | undefined;
  static new(width: number, height: number, answer_groups: Answer[]): AnswerGrid;
  /**
   * Handles the creation and updating of elements within the webpage.
   * 
   * Creates the `grid-container` div on the webpage and populates it with `Square` instances
   * containing the answer information.
   */
  setup(): void;
  width(): number;
  height(): number;
  get_idx(width: number, height: number): number;
}
export class Square {
  private constructor();
  free(): void;
  answer_group(): AnswerGroup;
  update_color(): void;
  id(): string;
  static new(x: number, y: number, answer_group: AnswerGroup, text: string): Square;
  text(): string;
  solve(): void;
  state(): SquareState;
  toggle(): void;
}
