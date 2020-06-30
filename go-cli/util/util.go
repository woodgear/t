package util

import (
	"time"

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

var  sugar *zap.SugaredLogger

func SyslogTimeEncoder(t time.Time, enc zapcore.PrimitiveArrayEncoder) {
	enc.AppendString(t.Format("Jan  2 15:04:05"))
}

func CustomLevelEncoder(level zapcore.Level, enc zapcore.PrimitiveArrayEncoder) {
	enc.AppendString("[" + level.CapitalString() + "]")
}

func Log() *zap.SugaredLogger{
	if sugar!=nil {
		return sugar
	}
	cfg := zap.Config{
		Encoding:    "console",
		OutputPaths: []string{"stderr"},
		EncoderConfig: zapcore.EncoderConfig{
			MessageKey:  "message",
			TimeKey:     "time",
			EncodeTime:  zapcore.ISO8601TimeEncoder,
			LevelKey:    "level",
			EncodeLevel: zapcore.CapitalLevelEncoder,
		},
	}

	cfg.Level = zap.NewAtomicLevelAt(zapcore.DebugLevel)

	logger, _ := cfg.Build()
	cfg.EncoderConfig.EncodeLevel = CustomLevelEncoder

	logger, _ = cfg.Build()
	sugar := logger.Sugar()
	return sugar
}
