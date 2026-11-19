export interface TrackRow { id: string; name: string; color?: string }

export function Timeline({ tracks }: { tracks: TrackRow[] }) {
  return (
    <div className="timeline" role="list">
      {tracks.map((track) => (
        <div className="timeline-row" role="listitem" key={track.id}>
          <div className="track-label">{track.name}</div>
          <div className="track-lane" style={{ background: track.color ?? '#171a21' }} />
        </div>
      ))}
    </div>
  )
}
