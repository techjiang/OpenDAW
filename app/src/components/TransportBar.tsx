export type TransportState = 'stopped' | 'playing' | 'paused' | 'recording'

export function TransportBar({ state, onPlay, onPause, onStop, onRecord }: {
  state: TransportState
  onPlay: () => void
  onPause: () => void
  onStop: () => void
  onRecord: () => void
}) {
  return (
    <div className="transport-bar">
      <button onClick={onRecord} aria-label="record">●</button>
      <button onClick={onPlay} aria-label="play">▶</button>
      <button onClick={onPause} aria-label="pause">⏸</button>
      <button onClick={onStop} aria-label="stop">■</button>
      <span data-state={state}>{state}</span>
    </div>
  )
}
