// Code generated by MockGen. DO NOT EDIT.
// Source: pkg/providers/doppler.go
//
// Generated by this command:
//
//	mockgen -source pkg/providers/doppler.go -destination pkg/providers/mock_providers/doppler_mock.go
//

// Package mock_providers is a generated GoMock package.
package mock_providers

import (
	reflect "reflect"

	http "github.com/DopplerHQ/cli/pkg/http"
	gomock "go.uber.org/mock/gomock"
)

// MockDopplerClient is a mock of DopplerClient interface.
type MockDopplerClient struct {
	ctrl     *gomock.Controller
	recorder *MockDopplerClientMockRecorder
}

// MockDopplerClientMockRecorder is the mock recorder for MockDopplerClient.
type MockDopplerClientMockRecorder struct {
	mock *MockDopplerClient
}

// NewMockDopplerClient creates a new mock instance.
func NewMockDopplerClient(ctrl *gomock.Controller) *MockDopplerClient {
	mock := &MockDopplerClient{ctrl: ctrl}
	mock.recorder = &MockDopplerClientMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use.
func (m *MockDopplerClient) EXPECT() *MockDopplerClientMockRecorder {
	return m.recorder
}

// GetSecrets mocks base method.
func (m *MockDopplerClient) GetSecrets(host string, verifyTLS bool, apiKey, project, config string) ([]byte, http.Error) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetSecrets", host, verifyTLS, apiKey, project, config)
	ret0, _ := ret[0].([]byte)
	ret1, _ := ret[1].(http.Error)
	return ret0, ret1
}

// GetSecrets indicates an expected call of GetSecrets.
func (mr *MockDopplerClientMockRecorder) GetSecrets(host, verifyTLS, apiKey, project, config any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetSecrets", reflect.TypeOf((*MockDopplerClient)(nil).GetSecrets), host, verifyTLS, apiKey, project, config)
}
