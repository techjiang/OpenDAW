export function WaveformView({ peaks }: { peaks: number[] }) {
  return (
    <svg className="waveform" viewBox={`0 0 ${peaks.length} 100`} preserveAspectRatio="none">
      {peaks.map((peak, index) => (
        <rect key={index} x={index} y={50 - peak * 50} width={1} height={peak * 100} fill="#5ad1a8" />
      ))}
    </svg>
  )
}
